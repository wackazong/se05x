// Copyright (C) 2023 Nitrokey GmbH
// SPDX-License-Identifier: LGPL-3.0-only

use core::{array::TryFromSliceError, convert::Infallible, fmt::Debug};

use crate::embedded_hal::Delay;
use bitflags::bitflags;
use delog::hexstr;
use hex_literal::hex;
use iso7816::{
    command::{
        class::{NO_SM_CLA, SM_CLA, ZERO_CLA},
        writer::IntoWriter,
        CommandBuilder, DataSource, DataStream, ExpectedLen, Writer,
    },
    tlv::{take_data_object, Tag, Tlv},
    Instruction, Status,
};

use crate::t1::{self, DataReceived, FrameSender, I2CForT1, T1oI2C};

use self::commands::{CreateEcCurve, SetEcCurveParam};

pub mod commands;

pub mod constants;
pub mod policies;

pub struct Se05X<Twi, D> {
    t1: T1oI2C<Twi, D>,
}

pub const MAX_APDU_PAYLOAD_LENGTH: usize = 889;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unknown,
    Line(u32),
    T1(t1::Error),
    Status(Status),
    Tlv,
}

impl From<Infallible> for Error {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}
impl From<TryFromSliceError> for Error {
    fn from(_value: TryFromSliceError) -> Self {
        Self::Line(line!())
    }
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Status(status) => status,
            Error::Unknown => Status::from(0x0000),
            Error::Tlv => Status::from(0x0001),
            Error::T1(t1::Error::Unknown) => Status::from(0x0002),
            Error::T1(t1::Error::AddressNack) => Status::from(0x0003),
            Error::T1(t1::Error::DataNack) => Status::from(0x0004),
            Error::T1(t1::Error::BadCrc) => Status::from(0x0005),
            Error::T1(t1::Error::BadPcb) => Status::from(0x0006),
            Error::T1(t1::Error::BadAddress) => Status::from(0x0007),
            Error::T1(t1::Error::ReceptionBuffer) => Status::from(0x0008),
            Error::T1(t1::Error::Timeout) => Status::from(0x0009),
            Error::T1(t1::Error::Line(l)) => Status::from(0x1000 + l.min(0x0FFF) as u16),
            Error::Line(l) => Status::from(0x2000 + l.min(0x0FFF) as u16),
        }
    }
}

impl From<t1::Error> for Error {
    fn from(value: t1::Error) -> Self {
        Self::T1(value)
    }
}

pub trait Se05XResponse<'a>: Sized {
    fn from_response(data: &'a [u8]) -> Result<Self, Error>;
}

impl<'a> Se05XResponse<'a> for () {
    fn from_response(_data: &'a [u8]) -> Result<Self, Error> {
        Ok(())
    }
}

pub trait Se05XCommand<W: Writer>: DataStream<W> {
    type Response<'a>: Se05XResponse<'a>;
}

impl<W: Writer, C: Se05XCommand<W>> Se05XCommand<W> for &C {
    type Response<'a> = C::Response<'a>;
}

pub const APP_ID: [u8; 0x10] = hex!("A0000003965453000000010300000000");

#[cfg(feature = "embedded-hal-v0.2.7")]
impl<M, N, E> Se05X<crate::embedded_hal::Hal027<M>, crate::embedded_hal::Hal027<N>>
where
    N: embedded_hal_v0_2_7::blocking::delay::DelayUs<u32>,
    M: embedded_hal_v0_2_7::blocking::i2c::Write<Error = E>
        + embedded_hal_v0_2_7::blocking::i2c::Read<Error = E>
        + embedded_hal_v0_2_7::blocking::i2c::WriteRead<Error = E>,
    E: crate::t1::I2CErrorNack,
{
    pub fn new_hal_027(twi: M, se_address: u8, delay: N) -> Self {
        Self::new(
            crate::embedded_hal::Hal027(twi),
            se_address,
            crate::embedded_hal::Hal027(delay),
        )
    }
}

#[cfg(feature = "embedded-hal-v1.0")]
impl<M, N, E> Se05X<crate::embedded_hal::Hal10<M>, crate::embedded_hal::Hal10<N>>
where
    N: embedded_hal_v1_0::delay::DelayNs,
    M: embedded_hal_v1_0::i2c::I2c<Error = E>,
    E: crate::t1::I2CErrorNack,
{
    pub fn new_hal_10(twi: M, se_address: u8, delay: N) -> Self {
        Self::new(
            crate::embedded_hal::Hal10(twi),
            se_address,
            crate::embedded_hal::Hal10(delay),
        )
    }
}

impl<Twi: I2CForT1, D: Delay> Se05X<Twi, D> {
    pub fn new(twi: Twi, se_address: u8, delay: D) -> Self {
        Self {
            t1: T1oI2C::new(twi, se_address, delay),
        }
    }

    pub fn set_t1_retry_count(&mut self, value: u32) {
        self.t1.retry_count = value;
    }

    fn receive_apdu<'buf>(
        &mut self,
        buffer: &'buf mut [u8],
    ) -> Result<(&'buf [u8], Status), Error> {
        match self.t1.receive_data(buffer)? {
            DataReceived::IBlocks(len) if len >= 2 => Ok((
                &buffer[..len - 2],
                Status::from([buffer[len - 2], buffer[len - 1]]),
            )),
            DataReceived::SBlock {
                block: _,
                i_data: _,
                s_data: _,
            } => Err(Error::Line(line!())),
            _ => {
                error!("Got too short apdu");
                Err(Error::Line(line!()))
            }
        }
    }

    pub fn enable(&mut self) -> Result<Atr, Error> {
        self.t1.resync()?;
        self.t1.interface_soft_reset(&mut [0; 64])?;
        let mut resp_buffer = [0; 9];
        let atr = self.run_command(&Select, &mut resp_buffer)?;
        debug!("Got ATR: {atr:02x?}");
        Ok(atr)
    }

    fn run_command_buf_response<'buf>(
        &mut self,
        command: &dyn for<'a> DataStream<FrameSender<'a, Twi, D>>,
        response_buf: &'buf mut [u8],
    ) -> Result<&'buf [u8], Error> {
        let mut sender = self.t1.into_writer(command.len())?;
        command.to_writer(&mut sender)?;
        self.t1.wait_segt();
        let (response, status) = self.receive_apdu(response_buf)?;
        if status != Status::Success {
            return Err(Error::Status(status));
        }
        Ok(response)
    }

    fn run_command_internal<'buf, R: Se05XResponse<'buf>>(
        &mut self,
        command: &dyn for<'a> DataStream<FrameSender<'a, Twi, D>>,
        response_buf: &'buf mut [u8],
    ) -> Result<R, Error> {
        let response = self.run_command_buf_response(command, response_buf)?;

        R::from_response(response)
    }

    pub fn run_command<'buf, C: for<'a> Se05XCommand<FrameSender<'a, Twi, D>>>(
        &mut self,
        command: &C,
        response_buf: &'buf mut [u8],
    ) -> Result<<C as Se05XCommand<FrameSender<'_, Twi, D>>>::Response<'buf>, Error> {
        self.run_command_internal(command, response_buf)
    }

    /// Run a command within a session
    pub fn run_session_command<'buf, C: for<'a> Se05XCommand<FrameSender<'a, Twi, D>>>(
        &mut self,
        session_id: SessionId,
        command: &C,
        response_buf: &'buf mut [u8],
    ) -> Result<<C as Se05XCommand<FrameSender<'_, Twi, D>>>::Response<'buf>, Error> {
        self.run_command_internal(
            &ProcessSessionCmd::<&dyn for<'a> DataStream<FrameSender<'a, Twi, D>>> {
                session_id,
                apdu: command,
            },
            response_buf,
        )
    }

    /// Prior to being used with the se05x, the curve constants need to be configured for the secure element
    ///
    /// This method configures the secure element to be able to use the given curve.
    ///
    /// The values for the `data` parameter can be found in the [`constants`]() module
    pub fn create_and_set_curve_params(
        &mut self,
        data: &constants::CurveInitializer,
    ) -> Result<(), Error> {
        let response_buf = &mut [0; 2];
        self.run_command(&CreateEcCurve { curve: data.curve }, response_buf)?;
        self.run_command(
            &SetEcCurveParam {
                curve: data.curve,
                param: EcCurveParam::ParamA,
                value: data.constants.a,
            },
            response_buf,
        )?;
        self.run_command(
            &SetEcCurveParam {
                curve: data.curve,
                param: EcCurveParam::ParamB,
                value: data.constants.b,
            },
            response_buf,
        )?;
        self.run_command(
            &SetEcCurveParam {
                curve: data.curve,
                param: EcCurveParam::ParamG,
                value: data.constants.g,
            },
            response_buf,
        )?;
        self.run_command(
            &SetEcCurveParam {
                curve: data.curve,
                param: EcCurveParam::ParamN,
                value: data.constants.order,
            },
            response_buf,
        )?;
        self.run_command(
            &SetEcCurveParam {
                curve: data.curve,
                param: EcCurveParam::ParamPrime,
                value: data.constants.prime,
            },
            response_buf,
        )?;
        Ok(())
    }

    #[deprecated(
        since = "0.1.3",
        note = "please use `create_and_set_curve_params` instead"
    )]
    /// Prior to being used with the se05x, the curve constants need to be configured for the secure element
    ///
    /// This method configures the secure element to be able to use the given curve.
    ///
    /// Since the commands are hard-coded, the constants can be quite large. If only a subset of the curves are used,
    /// it is recommended to instead use [`create_and_set_curve_params`]()
    pub fn create_and_set_curve(&mut self, curve: EcCurve) -> Result<(), Error> {
        let Some(constants) = curve.params() else {
            // Curve doesn't need configuring params
            return Ok(());
        };
        self.create_and_set_curve_params(&constants::CurveInitializer { constants, curve })
    }

    #[cfg(feature = "aes-session")]
    pub fn authenticate_aes128_session<R: rand::CryptoRng + rand::RngCore>(
        &mut self,
        session_id: SessionId,
        key: &[u8; 16],
        rng: &mut R,
    ) -> Result<bool, Error> {
        debug_now!("authenticating AES session");
        let mut buf = [0; 1024];
        use aes::Aes128;
        use cmac::{Cmac, Mac};
        use rand::Rng;

        use crate::se05x::commands::{ScpExternalAuthenticate, ScpInitializeUpdate};
        let host_challenge: [u8; 8] = rng.gen();
        let chal = self.run_session_command(
            session_id,
            &ScpInitializeUpdate { host_challenge },
            &mut buf,
        )?;
        debug_now!("InitializeUpdate successful");

        // *** Calculating keys *** //

        /// Data Derivation to generate Sess ENC Key
        const DATA_DERIVATION_SENC: u8 = 0x04;
        /// Data Derivation to generate Sess MAC Key
        const DATA_DERIVATION_SMAC: u8 = 0x06;
        /// Data Derivation to generate Sess RMAC Key
        const DATA_DERIVATION_SRMAC: u8 = 0x07;
        const DATA_DERIVATION_L_128_BIT: u16 = 0x0080;
        const DATA_DERIVATION_L_128_BIT_BE: [u8; 2] = DATA_DERIVATION_L_128_BIT.to_be_bytes();
        const DATA_DERIVATION_KDF_CTR: u8 = 0x01;

        let mut context = [0u8; 16];
        context[..8].copy_from_slice(&host_challenge);
        context[8..][..8].copy_from_slice(&chal.se05x_challenge.card_challenge);
        let mut dda = [0u8; 12 + 4 + 16];
        dda[12 + 1] = DATA_DERIVATION_L_128_BIT_BE[0];
        dda[12 + 2] = DATA_DERIVATION_L_128_BIT_BE[1];
        dda[12 + 3] = DATA_DERIVATION_KDF_CTR;
        dda[12 + 4..][..16].copy_from_slice(&context);

        dda[11] = DATA_DERIVATION_SENC;
        let mut mac = Cmac::<Aes128>::new(key.into());
        mac.update(&dda);
        let _tag_senc: &[u8; 16] = &mac.finalize().into_bytes().into();

        dda[11] = DATA_DERIVATION_SMAC;
        let mut mac = Cmac::<Aes128>::new(key.into());
        mac.update(&dda);
        let tag_smac: &[u8; 16] = &mac.finalize().into_bytes().into();

        dda[11] = DATA_DERIVATION_SRMAC;
        let mut mac = Cmac::<Aes128>::new(key.into());
        mac.update(&dda);
        let _tag_srmac: &[u8; 16] = &mac.finalize().into_bytes().into();

        // *** Verifying card cryptogram *** //
        const DATA_CARD_CRYPTOGRAM: u8 = 0;
        const DATA_HOST_CRYPTOGRAM: u8 = 1;
        const DATA_DERIVATION_L_64_BIT: u16 = 0x0040;
        const DATA_DERIVATION_L_64_BIT_BE: [u8; 2] = DATA_DERIVATION_L_64_BIT.to_be_bytes();

        dda[12 + 1] = DATA_DERIVATION_L_64_BIT_BE[0];
        dda[12 + 2] = DATA_DERIVATION_L_64_BIT_BE[1];

        dda[11] = DATA_CARD_CRYPTOGRAM;
        let mut mac = Cmac::<Aes128>::new(tag_smac.into());
        mac.update(&dda);
        let calculated_card_cryptogram: [u8; 16] = mac.finalize().into_bytes().into();
        if calculated_card_cryptogram[..8] != chal.se05x_challenge.card_cryptogram {
            debug_now!(
                "{dda:02x?} {host_challenge:02x?} {:02x?} {:02x?} {calculated_card_cryptogram:02x?}",
                chal.se05x_challenge.card_challenge,
                chal.se05x_challenge.card_cryptogram
            );
            return Ok(false);
        }

        debug_now!("Verified card cryptogram");

        dda[11] = DATA_HOST_CRYPTOGRAM;
        let mut mac = Cmac::<Aes128>::new(tag_smac.into());
        mac.update(&dda);
        let host_cryptogram: [u8; 16] = mac.finalize().into_bytes().into();
        let host_cryptogram: [u8; 8] = host_cryptogram[..8].try_into().unwrap();

        let mut mac = Cmac::<Aes128>::new(tag_smac.into());
        mac.update(&[0; 16]);
        // APDU header
        // FIXME: Secure messaging should be handled by `run_command`
        // BLOCKING: Expected len is not authenticated, so need adapted API from CommandBuilder
        mac.update(&hex!("84 82 0000 10"));
        mac.update(&host_cryptogram);

        debug_now!("Running external authenticate");
        self.run_session_command(
            session_id,
            &ScpExternalAuthenticate {
                host_cryptogram,
                mac: mac.finalize().into_bytes()[..8].try_into().unwrap(),
            },
            &mut buf,
        )?;
        debug_now!("Authenticate success");
        Ok(true)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AppletConfig: u16 {
        const ECDAA = 0x0001;
        const ECDSA_ECDH_ECDHE = 0x0002;
        const EDDSA = 0x0004;
        const DH_MONT = 0x0008;
        const HMAC = 0x0010;
        const RSA_PLAIN = 0x0020;
        const RSA_CRT = 0x0040;
        const AES = 0x0080;
        const DES = 0x0100;
        const PBKDF = 0x0200;
        const TLS = 0x0400;
        const MIFARE = 0x0800;
        const FIPS_MODE_DISABLED = 0x1000;
        const I2CM = 0x2000;
        const ECC_ALL = 0x000F;
        const RSA_ALL = 0x0060;
        const ALL = 0x3FFF;
    }
}

pub struct Select;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Atr {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub secure_box_major: u8,
    pub secure_box_minor: u8,
    pub applet_config: AppletConfig,
}

impl Atr {
    fn parse(atr: &[u8]) -> Result<Self, Error> {
        debug!("Parsing SELECT atr");
        let [major, minor, patch, config1, config2, secure_box_major, secure_box_minor] = atr
        else {
            return Err(Error::Line(line!()));
        };

        let applet_config =
            AppletConfig::from_bits_retain(u16::from_be_bytes([*config1, *config2]));
        Ok(Atr {
            major: *major,
            minor: *minor,
            patch: *patch,
            secure_box_major: *secure_box_major,
            secure_box_minor: *secure_box_minor,
            applet_config,
        })
    }
}

impl<'a> Se05XResponse<'a> for Atr {
    fn from_response(data: &'a [u8]) -> Result<Self, Error> {
        Self::parse(data)
    }
}

impl<'a> TryFrom<&'a [u8]> for Atr {
    type Error = Error;
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

pub type VersionInfo = Atr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectAttributes {
    identifier: ObjectId,
    class: SecureObjectType,
    authentication_indicator: SetIndicator,
    authentication_attempts_counter: u16,
    authentication_object_identifier: ObjectId,
    max_authentication_attempts: u16,
}

impl ObjectAttributes {
    fn parse(data: &[u8]) -> Result<Self, Error> {
        let [obj_id0, obj_id1, obj_id2, obj_id3, class, auth_indicator, attempts_counter0, attempts_counter1, auth_obj_id0, auth_obj_id1, auth_obj_id2, auth_obj_id3, max_auth_attempts0, max_auth_attempts1, _policy @ ..] =
            data
        else {
            return Err(Error::Line(line!()));
        };

        Ok(Self {
            identifier: ObjectId([*obj_id0, *obj_id1, *obj_id2, *obj_id3]),
            class: (*class).try_into().map_err(|_| Error::Line(line!()))?,
            authentication_indicator: (*auth_indicator)
                .try_into()
                .map_err(|_| Error::Line(line!()))?,
            authentication_attempts_counter: u16::from_be_bytes([
                *attempts_counter0,
                *attempts_counter1,
            ]),
            authentication_object_identifier: ObjectId([
                *auth_obj_id0,
                *auth_obj_id1,
                *auth_obj_id2,
                *auth_obj_id3,
            ]),
            max_authentication_attempts: u16::from_be_bytes([
                *max_auth_attempts0,
                *max_auth_attempts1,
            ]),
        })
    }

    pub fn identifier(&self) -> ObjectId {
        self.identifier
    }
    pub fn class(&self) -> SecureObjectType {
        self.class
    }
    pub fn authentication_indicator(&self) -> SetIndicator {
        self.authentication_indicator
    }
    pub fn authentication_attempts_counter(&self) -> u16 {
        self.authentication_attempts_counter
    }
    pub fn authentication_object_identifier(&self) -> ObjectId {
        self.authentication_object_identifier
    }
    pub fn max_authentication_attempts(&self) -> u16 {
        self.max_authentication_attempts
    }
}

impl<'a> Se05XResponse<'a> for ObjectAttributes {
    fn from_response(data: &'a [u8]) -> Result<Self, Error> {
        Self::parse(data)
    }
}

impl<'a> TryFrom<&'a [u8]> for ObjectAttributes {
    type Error = Error;
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl Select {
    fn command(&self) -> CommandBuilder<&'static [u8]> {
        CommandBuilder::new(ZERO_CLA, 0xA4.into(), 0x04, 0x00, &APP_ID, 7)
    }
}

impl DataSource for Select {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        false
    }
}

impl<W: Writer> DataStream<W> for Select {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for Select {
    type Response<'a> = Atr;
}

pub struct ProcessSessionCmd<C> {
    pub session_id: SessionId,
    pub apdu: C,
}

impl<C: DataSource> ProcessSessionCmd<C> {
    fn data(&self) -> (Tlv<SessionId>, Tlv<&C>) {
        (
            Tlv::new(TAG_SESSION_ID, self.session_id),
            Tlv::new(TAG_1, &self.apdu),
        )
    }
    fn command(&self) -> CommandBuilder<(Tlv<SessionId>, Tlv<&C>)> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_PROCESS,
            P1_DEFAULT,
            P2_DEFAULT,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<C: DataSource> DataSource for ProcessSessionCmd<C> {
    fn len(&self) -> usize {
        self.command().len()
    }

    fn is_empty(&self) -> bool {
        false
    }
}

impl<W: Writer, C: DataStream<W>> DataStream<W> for ProcessSessionCmd<C> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer, C: Se05XCommand<W>> Se05XCommand<W> for ProcessSessionCmd<C> {
    type Response<'a> = C::Response<'a>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Se05xChallenge {
    pub key_diversification_data: [u8; 10],
    pub key_information: [u8; 3],
    pub card_challenge: [u8; 8],
    pub card_cryptogram: [u8; 8],
}

impl From<&[u8; 29]> for Se05xChallenge {
    fn from(value: &[u8; 29]) -> Self {
        let (key_diversification_data, rem) = value.split_at(10);
        let (key_information, rem) = rem.split_at(3);
        let (card_challenge, rem) = rem.split_at(8);
        let (card_cryptogram, rem) = rem.split_at(8);
        assert!(rem.is_empty());
        Self {
            key_diversification_data: key_diversification_data.try_into().unwrap(),
            key_information: key_information.try_into().unwrap(),
            card_challenge: card_challenge.try_into().unwrap(),
            card_cryptogram: card_cryptogram.try_into().unwrap(),
        }
    }
}

impl TryFrom<&[u8]> for Se05xChallenge {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 29 {
            return Err(Error::Line(line!()));
        }
        debug_now!("Challenge value: {:02x?}", value);
        debug_now!("Challenge len: {}", value.len());
        let value: &[u8; 29] = value[..29].try_into()?;
        Ok(value.into())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CryptoObjectId(#[cfg_attr(feature = "serde", serde(with = "serde_bytes"))] pub [u8; 2]);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionId(#[cfg_attr(feature = "serde", serde(with = "serde_bytes"))] pub [u8; 8]);

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectId(#[cfg_attr(feature = "serde", serde(with = "serde_bytes"))] pub [u8; 4]);

impl Debug for ObjectId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ObjectId").field(&hexstr!(&self.0)).finish()
    }
}

impl ObjectId {
    /// Invalid object ID.
    /// Can be used in policy to configure no-session access
    pub const INVALID: ObjectId = ObjectId(hex!("00000000"));
    /// An authentication object which allows the user to switch
    /// LockState of the applet. The LockState defines whether the
    /// applet is transport locked or not.
    pub const TRANSPORT: ObjectId = ObjectId(hex!("7FFF0200"));
    /// A device unique NIST P-256 key pair rwhich contains SK.SE.ECKA
    /// and PK.SE.ECKA in ECKey session context.
    pub const KP_ECKEY_USER: ObjectId = ObjectId(hex!("7FFF0201"));
    /// A device unique NIST P-256 key pair which contains SK.SE.ECKA
    /// and PK.SE.ECKA in ECKey session context; A constant card
    /// challenge (all zeroes) is applicable.
    pub const KP_ECKEY_IMPORT: ObjectId = ObjectId(hex!("7FFF0202"));
    // Reserved Key @ location 0x7FFF0203
    /// An authentication object which allows the user to change the
    /// applet variant.
    pub const FEATURE: ObjectId = ObjectId(hex!("7FFF0204"));
    /// An authentication object which allows the user to delete all
    /// objects, except trust provisioned by NXP objects.
    pub const FACTORY_RESET: ObjectId = ObjectId(hex!("7FFF0205"));
    /// A BinaryFile Secure Object which holds the device unique
    ///  ID. This file cannot be overwritten or deleted.
    pub const UNIQUE_ID: ObjectId = ObjectId(hex!("7FFF0206"));
    /// An authentication object which allows the user to change the
    /// platform SCP requirements, i.e. make platform SCP mandatory or
    /// not, using SetPlatformSCPRequest. Mandatory means full security,
    /// i.e. command & response MAC and encryption. Only SCP03 will be
    /// sufficient.
    pub const PLATFORM_SCP: ObjectId = ObjectId(hex!("7FFF0207"));
    /// An authentication object which grants access to the I2C master
    /// feature. If the credential is not present, access to I2C master
    /// is allowed in general. Otherwise, a session using this
    /// credential shall be established and I2CM commands shall be sent
    /// within this session.
    pub const I2CM_ACCESS: ObjectId = ObjectId(hex!("7FFF0208"));
    /// An authentication object which grants access to the
    /// SetLockState command
    pub const RESTRICT: ObjectId = ObjectId(hex!("7FFF020A"));
}

impl TryFrom<&[u8]> for ObjectId {
    type Error = TryFromSliceError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let tmp = value.try_into()?;
        Ok(Self(tmp))
    }
}

impl TryFrom<&[u8]> for SessionId {
    type Error = TryFromSliceError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let tmp = value.try_into()?;
        Ok(Self(tmp))
    }
}

impl TryFrom<&[u8]> for CryptoObjectId {
    type Error = TryFromSliceError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let tmp = value.try_into()?;
        Ok(Self(tmp))
    }
}

impl DataSource for ObjectId {
    fn len(&self) -> usize {
        4
    }

    fn is_empty(&self) -> bool {
        false
    }
}

impl DataSource for CryptoObjectId {
    fn len(&self) -> usize {
        2
    }

    fn is_empty(&self) -> bool {
        false
    }
}

impl DataSource for SessionId {
    fn len(&self) -> usize {
        8
    }

    fn is_empty(&self) -> bool {
        false
    }
}

impl<W: Writer> DataStream<W> for CryptoObjectId {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        writer.write_all(&self.0)
    }
}
impl<W: Writer> DataStream<W> for SessionId {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        writer.write_all(&self.0)
    }
}
impl<W: Writer> DataStream<W> for ObjectId {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        writer.write_all(&self.0)
    }
}

pub const TAG_SESSION_ID: Tag = Tag::from_u8(0x10);
pub const TAG_POLICY: Tag = Tag::from_u8(0x11);
pub const TAG_MAX_ATTEMPTS: Tag = Tag::from_u8(0x12);
pub const TAG_IMPORT_AUTH_DATA: Tag = Tag::from_u8(0x13);
pub const TAG_IMPORT_AUTH_KEY_ID: Tag = Tag::from_u8(0x14);
pub const TAG_1: Tag = Tag::from_u8(0x41);
pub const TAG_2: Tag = Tag::from_u8(0x42);
pub const TAG_3: Tag = Tag::from_u8(0x43);
pub const TAG_4: Tag = Tag::from_u8(0x44);
pub const TAG_5: Tag = Tag::from_u8(0x45);
pub const TAG_6: Tag = Tag::from_u8(0x46);
pub const TAG_7: Tag = Tag::from_u8(0x47);
pub const TAG_8: Tag = Tag::from_u8(0x48);
pub const TAG_9: Tag = Tag::from_u8(0x49);
pub const TAG_10: Tag = Tag::from_u8(0x4A);

pub const INS_TRANSIENT: Instruction = Instruction::Unknown(0x80);
pub const INS_AUTH_OBJECT: Instruction = Instruction::Unknown(0x40);
pub const INS_ATTEST: Instruction = Instruction::Unknown(0x20);

pub const INS_WRITE: Instruction = Instruction::Unknown(0x01);
pub const INS_READ: Instruction = Instruction::Unknown(0x02);
pub const INS_READ_ATTEST: Instruction = Instruction::Unknown(0x02 | 0x20);
pub const INS_CRYPTO: Instruction = Instruction::Unknown(0x03);
pub const INS_MGMT: Instruction = Instruction::Unknown(0x04);
pub const INS_PROCESS: Instruction = Instruction::Unknown(0x05);
pub const INS_IMPORT_EXTERNAL: Instruction = Instruction::Unknown(0x06);

pub const INS_INITIALIZE_UPDATE: Instruction = Instruction::Unknown(0x50);
pub const INS_EXTERNAL_AUTHENTICATE: Instruction = Instruction::Unknown(0x82);

/// Highest bit not used
pub const P1_UNUSED: u8 = 0x80;
/// 2 MSBit for key type
pub const P1_MASK_KEY_TYPE: u8 = 0x60;
/// 5 LSBit for credential type
pub const P1_MASK_CRED_TYPE: u8 = 0x1F;

/// Key pair (private key + public key)
pub const P1_KEY_PAIR: u8 = 0x60;
/// Private key
pub const P1_PRIVATE: u8 = 0x40;
/// Public key
pub const P1_PUBLIC: u8 = 0x20;

pub const P1_DEFAULT: u8 = 0x00;
pub const P1_EC: u8 = 0x01;
pub const P1_RSA: u8 = 0x02;
pub const P1_AES: u8 = 0x03;
pub const P1_DES: u8 = 0x04;
pub const P1_HMAC: u8 = 0x05;
pub const P1_BINARY: u8 = 0x06;
pub const P1_USERID: u8 = 0x07;
pub const P1_COUNTER: u8 = 0x08;
pub const P1_PCR: u8 = 0x09;
pub const P1_CURVE: u8 = 0x0B;
pub const P1_SIGNATURE: u8 = 0x0C;
pub const P1_MAC: u8 = 0x0D;
pub const P1_CIPHER: u8 = 0x0E;
pub const P1_TLS: u8 = 0x0F;
pub const P1_CRYPTO_OBJ: u8 = 0x10;

pub const P2_DEFAULT: u8 = 0x00;
pub const P2_GENERATE: u8 = 0x03;
pub const P2_CREATE: u8 = 0x04;
pub const P2_SIZE: u8 = 0x07;
pub const P2_SIGN: u8 = 0x09;
pub const P2_VERIFY: u8 = 0x0A;
pub const P2_INIT: u8 = 0x0B;
pub const P2_UPDATE: u8 = 0x0C;
pub const P2_FINAL: u8 = 0x0D;
pub const P2_ONESHOT: u8 = 0x0E;
pub const P2_DH: u8 = 0x0F;
pub const P2_DIVERSIFY: u8 = 0x10;
pub const P2_AUTH_FIRST_PART2: u8 = 0x12;
pub const P2_AUTH_NONFIRST_PART2: u8 = 0x13;
pub const P2_DUMP_KEY: u8 = 0x14;
pub const P2_CHANGE_KEY_PART1: u8 = 0x15;
pub const P2_CHANGE_KEY_PART2: u8 = 0x16;
pub const P2_KILL_AUTH: u8 = 0x17;
pub const P2_IMPORT: u8 = 0x18;
pub const P2_EXPORT: u8 = 0x19;
pub const P2_SESSION_CREATE: u8 = 0x1B;
pub const P2_SESSION_CLOSE: u8 = 0x1C;
pub const P2_SESSION_REFRESH: u8 = 0x1E;
pub const P2_SESSION_POLICY: u8 = 0x1F;
pub const P2_VERSION: u8 = 0x20;
pub const P2_MEMORY: u8 = 0x22;
pub const P2_LIST: u8 = 0x25;
pub const P2_TYPE: u8 = 0x26;
pub const P2_EXIST: u8 = 0x27;
pub const P2_DELETE_OBJECT: u8 = 0x28;
pub const P2_DELETE_ALL: u8 = 0x2A;
pub const P2_SESSION_USERID: u8 = 0x2C;
pub const P2_HKDF: u8 = 0x2D;
pub const P2_PBKDF: u8 = 0x2E;
pub const P2_I2CM: u8 = 0x30;
pub const P2_I2CM_ATTESTED: u8 = 0x31;
pub const P2_MAC: u8 = 0x32;
pub const P2_UNLOCK_CHALLENGE: u8 = 0x33;
pub const P2_CURVE_LIST: u8 = 0x34;
pub const P2_SIGN_ECDAA: u8 = 0x35;
pub const P2_ID: u8 = 0x36;
pub const P2_ENCRYPT_ONESHOT: u8 = 0x37;
pub const P2_DECRYPT_ONESHOT: u8 = 0x38;
pub const P2_ATTEST: u8 = 0x3A;
pub const P2_ATTRIBUTES: u8 = 0x3B;
pub const P2_CPLC: u8 = 0x3C;
pub const P2_TIME: u8 = 0x3D;
pub const P2_TRANSPORT: u8 = 0x3E;
pub const P2_VARIANT: u8 = 0x3F;
pub const P2_PARAM: u8 = 0x40;
pub const P2_DELETE_CURVE: u8 = 0x41;
pub const P2_ENCRYPT: u8 = 0x42;
pub const P2_DECRYPT: u8 = 0x43;
pub const P2_VALIDATE: u8 = 0x44;
pub const P2_GENERATE_ONESHOT: u8 = 0x45;
pub const P2_VALIDATE_ONESHOT: u8 = 0x46;
pub const P2_CRYPTO_LIST: u8 = 0x47;
pub const P2_RANDOM: u8 = 0x49;
pub const P2_TLS_PMS: u8 = 0x4A;
pub const P2_TLS_PRF_CLI_HELLO: u8 = 0x4B;
pub const P2_TLS_PRF_SRV_HELLO: u8 = 0x4C;
pub const P2_TLS_PRF_CLI_RND: u8 = 0x4D;
pub const P2_TLS_PRF_SRV_RND: u8 = 0x4E;
pub const P2_RAW: u8 = 0x4F;
pub const P2_IMPORT_EXT: u8 = 0x51;
pub const P2_SCP: u8 = 0x52;
pub const P2_AUTH_FIRST_PART1: u8 = 0x53;
pub const P2_AUTH_NONFIRST_PART1: u8 = 0x54;

pub const TYPE_EC_KEY_PAIR: u8 = 0x01;
pub const TYPE_EC_PRIV_KEY: u8 = 0x02;
pub const TYPE_EC_PUB_KEY: u8 = 0x03;
pub const TYPE_RSA_KEY_PAIR: u8 = 0x04;
pub const TYPE_RSA_KEY_PAIR_CRT: u8 = 0x05;
pub const TYPE_RSA_PRIV_KEY: u8 = 0x06;
pub const TYPE_RSA_PRIV_KEY_CRT: u8 = 0x07;
pub const TYPE_RSA_PUB_KEY: u8 = 0x08;
pub const TYPE_AES_KEY: u8 = 0x09;
pub const TYPE_DES_KEY: u8 = 0x0A;
pub const TYPE_BINARY_FILE: u8 = 0x0B;
pub const TYPE_USERID: u8 = 0x0C;
pub const TYPE_COUNTER: u8 = 0x0D;
pub const TYPE_PCR: u8 = 0x0F;
pub const TYPE_CURVE: u8 = 0x10;
pub const TYPE_HMAC_KEY: u8 = 0x11;

pub const DIGEST_NO_HASH: u8 = 0x00;
pub const DIGEST_SHA: u8 = 0x01;
pub const DIGEST_SHA224: u8 = 0x07;
pub const DIGEST_SHA256: u8 = 0x04;
pub const DIGEST_SHA384: u8 = 0x05;
pub const DIGEST_SHA512: u8 = 0x06;

pub const HMAC_SHA1: u8 = 0x18;
pub const HMAC_SHA256: u8 = 0x19;
pub const HMAC_SHA384: u8 = 0x1A;
pub const HMAC_SHA512: u8 = 0x1B;
///  (ISO9797 M2 padding)
pub const CMAC_128: u8 = 0x31;
pub const DES_MAC4_ISO9797_M2: u8 = 0x05;
pub const DES_MAC4_ISO9797_1_M2_ALG3: u8 = 0x13;
pub const DES_MAC4_ISO9797_M1: u8 = 0x03;
pub const DES_MAC4_ISO9797_1_M1_ALG3: u8 = 0x2F;
pub const DES_MAC8_ISO9797_M2: u8 = 0x06;
pub const DES_MAC8_ISO9797_1_M2_ALG3: u8 = 0x14;
pub const DES_MAC8_ISO9797_1_M1_ALG3: u8 = 0x04;
// pub const DES_MAC8_ISO9797_1_M1_ALG3: u8 = 0x30;
pub const CMAC128: u8 = 0x31;
pub const DES_CMAC8: u8 = 0x7A;
pub const AES_CMAC16: u8 = 0x66;

pub const NIST_P192: u8 = 0x01;
pub const NIST_P224: u8 = 0x02;
pub const NIST_P256: u8 = 0x03;
pub const NIST_P384: u8 = 0x04;
pub const NIST_P521: u8 = 0x05;
pub const BRAINPOOL160: u8 = 0x06;
pub const BRAINPOOL192: u8 = 0x07;
pub const BRAINPOOL224: u8 = 0x08;
pub const BRAINPOOL256: u8 = 0x09;
pub const BRAINPOOL320: u8 = 0x0A;
pub const BRAINPOOL384: u8 = 0x0B;
pub const BRAINPOOL512: u8 = 0x0C;
pub const SECP160K1: u8 = 0x0D;
pub const SECP192K1: u8 = 0x0E;
pub const SECP224K1: u8 = 0x0F;
pub const SECP256K1: u8 = 0x10;
pub const TPM_ECC_BN_P256: u8 = 0x11;
pub const ID_ECC_ED_25519: u8 = 0x40;
pub const ID_ECC_MONT_DH_25519: u8 = 0x41;

pub const CURVE_PARAM_A: u8 = 0x01;
pub const CURVE_PARAM_B: u8 = 0x02;
pub const CURVE_PARAM_G: u8 = 0x04;
pub const CURVE_PARAM_N: u8 = 0x08;
pub const CURVE_PARAM_PRIME: u8 = 0x10;

pub const TRANSIENT_LOCK: u8 = 0x01;
pub const PERSISTENT_LOCK: u8 = 0x02;

pub const LOCKED: u8 = 0x01;

/// For DigestInit/DigestUpdate/DigestFinal
pub const CC_DIGEST: u8 = 0x01;
/// For CipherInit/CipherUpdate/CipherFinal
pub const CC_CIPHER: u8 = 0x02;
/// For MACInit/MACUpdate/MACFinal
pub const CC_SIGNATURE: u8 = 0x03;

pub const RESULT_SUCCESS: u8 = 0x01;
pub const RESULT_FAILURE: u8 = 0x02;

pub const PERSISTENT: u8 = 0x01;
pub const TRANSIENT: u8 = 0x02;

pub const NOT_SET: u8 = 0x01;
pub const SET: u8 = 0x02;

/// Persistent memory
pub const MEM_PERSISTENT: u8 = 0x01;
/// Transient memory, clear on reset
pub const MEM_TRANSIENT_RESET: u8 = 0x02;
/// Transient memory, clear on deselect
pub const MEM_TRANSIENT_DESELECT: u8 = 0x03;

/// Generated outside the module.
pub const ORIGIN_EXTERNAL: u8 = 0x01;
/// Generated inside the module.
pub const ORIGIN_INTERNAL: u8 = 0x02;
/// Trust provisioned by NXP
pub const ORIGIN_PROVISIONED: u8 = 0x03;

/// NOT SUPPORTED
pub const SIG_ECDSA_PLAIN: u8 = 0x09;
/// ECDSA with a SHA-1 digest as input.
pub const SIG_ECDSA_SHA: u8 = 0x11;
/// ECDSA with a SHA224 digest as input.
pub const SIG_ECDSA_SHA_224: u8 = 0x25;
/// ECDSA with a SHA256 digest as input.
pub const SIG_ECDSA_SHA_256: u8 = 0x21;
/// ECDSA with a SHA384 digest as input.
pub const SIG_ECDSA_SHA_384: u8 = 0x22;
/// ECDSA with a SHA512 digest as input.
pub const SIG_ECDSA_SHA_512: u8 = 0x26;

/// EDDSA Pure (using SHA512 as digest)
pub const SIG_ED25519PURE: u8 = 0xA3;

/// Message input must be pre-hashed (using SHA256)
pub const SIG_ECDAA: u8 = 0xF4;

/// RFC8017: RSASSA-PSS
pub const RSA_SHA1_PKCS1_PSS: u8 = 0x15;
/// RFC8017: RSASSA-PSS
pub const RSA_SHA224_PKCS1_PSS: u8 = 0x2B;
/// RFC8017: RSASSA-PSS
pub const RSA_SHA256_PKCS1_PSS: u8 = 0x2C;
/// RFC8017: RSASSA-PSS
pub const RSA_SHA384_PKCS1_PSS: u8 = 0x2D;
/// RFC8017: RSASSA-PSS
pub const RSA_SHA512_PKCS1_PSS: u8 = 0x2E;
/// RFC8017: RSASSA-PKCS1-v1_5
pub const RSA_SHA1_PKCS1: u8 = 0x0A;
/// RFC8017: RSASSA-PKCS1-v1_5
pub const RSA_SHA_224_PKCS1: u8 = 0x27;
/// RFC8017: RSASSA-PKCS1-v1_5
pub const RSA_SHA_256_PKCS1: u8 = 0x28;
/// RFC8017: RSASSA-PKCS1-v1_5
pub const RSA_SHA_384_PKCS1: u8 = 0x29;
/// RFC8017: RSASSA-PKCS1-v1_5
pub const RSA_SHA_512_PKCS1: u8 = 0x2A;

/// Plain RSA, padding required on host.
pub const RSA_NO_PAD: u8 = 0x0C;
/// RFC8017: RSAES-PKCS1-v1_5
pub const RSA_PKCS1: u8 = 0x0A;
/// RFC8017: RSAES-OAEP (using SHA1 as digest)
pub const RSA_PKCS1_OAEP: u8 = 0x0F;

/// Modulus
pub const RSA_COMP_MOD: u8 = 0x00;
/// Public key exponent
pub const RSA_COMP_PUB_EXP: u8 = 0x01;
/// Private key exponent
pub const RSA_COMP_PRIV_EXP: u8 = 0x02;
/// CRT component p
pub const RSA_COMP_P: u8 = 0x03;
/// CRT component q
pub const RSA_COMP_Q: u8 = 0x04;
/// CRT component dp
pub const RSA_COMP_DP: u8 = 0x05;
/// CRT component dq
pub const RSA_COMP_DQ: u8 = 0x06;
/// CRT component q_inv
pub const RSA_COMP_INVQ: u8 = 0x07;
/// Not applicable
pub const RSA_COMP_NA: u8 = 0xFF;

/// Typically using DESKey identifiers
pub const DES_CBC_NOPAD: u8 = 0x01;
/// Typically using DESKey identifiers
pub const DES_CBC_ISO9797_M1: u8 = 0x02;
/// Typically using DESKey identifiers
pub const DES_CBC_ISO9797_M2: u8 = 0x03;
/// NOT SUPPORTED
pub const DES_CBC_PKCS5: u8 = 0x04;
/// Typically using DESKey identifiers
pub const DES_ECB_NOPAD: u8 = 0x05;
/// NOT SUPPORTED
pub const DES_ECB_ISO9797_M1: u8 = 0x06;
/// NOT SUPPORTED
pub const DES_ECB_ISO9797_M2: u8 = 0x07;
/// NOT SUPPORTED
pub const DES_ECB_PKCS5: u8 = 0x08;
/// Typically using AESKey identifiers
pub const AES_ECB_NOPAD: u8 = 0x0E;
/// Typically using AESKey identifiers
pub const AES_CBC_NOPAD: u8 = 0x0D;
/// Typically using AESKey identifiers
pub const AES_CBC_ISO9797_M1: u8 = 0x16;
/// Typically using AESKey identifiers
pub const AES_CBC_ISO9797_M2: u8 = 0x17;
/// NOT SUPPORTED
pub const AES_CBC_PKCS5: u8 = 0x18;
/// Typically using AESKey identifiers
pub const AES_CTR: u8 = 0xF0;

/// No more data available
pub const NO_MORE: u8 = 0x01;
/// More data available
pub const MORE: u8 = 0x02;

/// Platform SCP is required (full enc & MAC)
pub const SCP_REQUIRED: u8 = 0x01;
/// No platform SCP required.
pub const SCP_NOT_REQUIRED: u8 = 0x02;

/// Big-endian encoded integer
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Be<I>(pub I);

impl<I> From<I> for Be<I> {
    fn from(value: I) -> Self {
        Self(value)
    }
}

impl DataSource for Be<u8> {
    fn len(&self) -> usize {
        1
    }
}
impl DataSource for Be<u16> {
    fn len(&self) -> usize {
        2
    }
}
impl DataSource for Be<u32> {
    fn len(&self) -> usize {
        4
    }
}
impl DataSource for Be<u64> {
    fn len(&self) -> usize {
        8
    }
}

impl<W: Writer> DataStream<W> for Be<u8> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        writer.write_all(&self.0.to_be_bytes())
    }
}
impl<W: Writer> DataStream<W> for Be<u16> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        writer.write_all(&self.0.to_be_bytes())
    }
}
impl<W: Writer> DataStream<W> for Be<u32> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        writer.write_all(&self.0.to_be_bytes())
    }
}
impl<W: Writer> DataStream<W> for Be<u64> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        writer.write_all(&self.0.to_be_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for Be<u8> {
    type Error = Error;
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let arr: &'a _ = value.try_into().map_err(|_| Error::Tlv)?;
        Ok(u8::from_be_bytes(*arr).into())
    }
}

impl<'a> TryFrom<&'a [u8]> for Be<u16> {
    type Error = Error;
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let arr: &'a _ = value.try_into().map_err(|_| Error::Tlv)?;
        Ok(u16::from_be_bytes(*arr).into())
    }
}

impl<'a> TryFrom<&'a [u8]> for Be<u32> {
    type Error = Error;
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let arr: &'a _ = value.try_into().map_err(|_| Error::Tlv)?;
        Ok(u32::from_be_bytes(*arr).into())
    }
}

impl<'a> TryFrom<&'a [u8]> for Be<u64> {
    type Error = Error;
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let arr: &'a _ = value.try_into().map_err(|_| Error::Tlv)?;
        Ok(u64::from_be_bytes(*arr).into())
    }
}

macro_rules! enum_data {
    (
        #[$outer:meta]
        #[repr($repr:tt)]
        $vis:vis enum $name:ident {
            $(
                $(#[$var_tag:meta])?
                $var:ident = $num:tt
            ),+
            $(,)*
        }
    ) => {
        #[$outer]
        #[repr($repr)]
        $vis enum $name {
            $(
                $(#[$var_tag])?
                $var = $num,
            )*
        }

        impl From<$name> for $repr {
            fn from(val: $name) -> $repr {
                match val {
                    $(
                         $name::$var => $num,
                    )*
                }
            }
        }

        impl TryFrom<$repr> for $name {
            type Error = ();
            fn try_from(val: $repr) -> ::core::result::Result<Self, ()> {
                match val {
                    $(
                        $num => Ok($name::$var),
                    )*
                    _ => Err(())
                }
            }
        }

        impl<'a> TryFrom<&'a [u8]> for $name {
            type Error = Error;
            fn try_from(val: &'a [u8]) -> ::core::result::Result<Self, Error> {
                let arr: &'a _ = val.try_into().map_err(|_| Error::Tlv)?;
                $repr::from_be_bytes(*arr).try_into().map_err(|_| Error::Tlv)
            }
        }

        impl DataSource for $name {
            fn len(&self) -> usize {
                $repr::default().to_be_bytes().len()
            }
            fn is_empty(&self) -> bool {
                false
            }
        }

        impl<W:Writer> DataStream<W> for $name {
            fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
                let val: $repr = (*self).into();
                writer.write_all(&val.to_be_bytes())
            }
        }

        impl<T : Into<$repr>> core::ops::BitOr<T> for $name {
            type Output = $repr;
        	fn bitor(self, rhs: T) -> $repr {
        		let a: $repr = self.into();
        		let b: $repr = rhs.into();
        		a | b
            }
        }

        impl<T : Into<$repr>> core::ops::BitAnd<T> for $name {
            type Output = $repr;
        	fn bitand(self, rhs: T) -> $repr {
        		let a: $repr = self.into();
        		let b: $repr = rhs.into();
        		a & b
            }
        }

    };
}

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum TransientIndicator {
        Transient = TRANSIENT_LOCK,
        Persistent = PERSISTENT_LOCK,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockState {
    Locked,
    Unlocked,
}

impl From<LockState> for u8 {
    fn from(value: LockState) -> Self {
        match value {
            LockState::Locked => LOCKED,
            LockState::Unlocked => 0x02,
        }
    }
}

impl From<u8> for LockState {
    fn from(value: u8) -> Self {
        match value {
            LOCKED => LockState::Locked,
            _ => LockState::Unlocked,
        }
    }
}

impl TryFrom<&[u8]> for LockState {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Error> {
        if let [b] = value {
            Ok((*b).into())
        } else {
            Err(Error::Tlv)
        }
    }
}
impl DataSource for LockState {
    fn len(&self) -> usize {
        1
    }
    fn is_empty(&self) -> bool {
        false
    }
}

impl<W: Writer> DataStream<W> for LockState {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        writer.write_all(&[(*self).into()])
    }
}

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum P1KeyType {
        Na = P1_DEFAULT,
        KeyPair = P1_KEY_PAIR,
        Private = P1_PRIVATE,
        Public = P1_PUBLIC,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum RsaFormat {
        Crt = P2_DEFAULT,
        Raw = P2_RAW,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum EcCurve {
        NistP192 = NIST_P192,
        NistP224 = NIST_P224,
        NistP256 = NIST_P256,
        NistP384 = NIST_P384,
        NistP521 = NIST_P521,
        Brainpool160 = BRAINPOOL160,
        Brainpool192 = BRAINPOOL192,
        Brainpool224 = BRAINPOOL224,
        Brainpool256 = BRAINPOOL256,
        Brainpool320 = BRAINPOOL320,
        Brainpool384 = BRAINPOOL384,
        Brainpool512 = BRAINPOOL512,
        Secp160k1 = SECP160K1,
        Secp192k1 = SECP192K1,
        Secp224k1 = SECP224K1,
        Secp256k1 = SECP256K1,
        TpmEccBnP256 = TPM_ECC_BN_P256,
        IdEccEd25519 = ID_ECC_ED_25519,
        IdEccMontDh25519 = ID_ECC_MONT_DH_25519,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum EcCurveParam {
        ParamA = CURVE_PARAM_A,
        ParamB = CURVE_PARAM_B,
        ParamG = CURVE_PARAM_G,
        ParamN = CURVE_PARAM_N,
        ParamPrime = CURVE_PARAM_PRIME,
    }
);

impl EcCurve {
    /// None means that the constant doesn't need configuring its parameters (curve 25519)
    pub fn params(&self) -> Option<constants::CurveConstants> {
        match self {
            Self::NistP192 => Some(constants::PRIME192V1),
            Self::NistP224 => Some(constants::SECP224R1),
            Self::NistP256 => Some(constants::PRIME256V1),
            Self::NistP384 => Some(constants::SECP384R1),
            Self::NistP521 => Some(constants::SECP521R1),

            Self::Brainpool160 => Some(constants::BRAINPOOL_P160R1),
            Self::Brainpool192 => Some(constants::BRAINPOOL_P192R1),
            Self::Brainpool224 => Some(constants::BRAINPOOL_P224R1),
            Self::Brainpool256 => Some(constants::BRAINPOOL_P256R1),
            Self::Brainpool320 => Some(constants::BRAINPOOL_P320R1),
            Self::Brainpool384 => Some(constants::BRAINPOOL_P384R1),
            Self::Brainpool512 => Some(constants::BRAINPOOL_P512R1),

            Self::Secp160k1 => Some(constants::SECP160K1),
            Self::Secp192k1 => Some(constants::SECP192K1),
            Self::Secp224k1 => Some(constants::SECP224K1),
            Self::Secp256k1 => Some(constants::SECP256K1),

            Self::TpmEccBnP256 => Some(constants::TPM_BN_P256),
            Self::IdEccEd25519 => None,
            Self::IdEccMontDh25519 => None,
        }
    }
}

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SymmKeyType {
        Aes = P1_AES,
        Des = P1_DES,
        Hmac = P1_HMAC,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u16)]
    pub enum CounterSize {
        B1 = 1,
        B2 = 2,
        B3 = 3,
        B4 = 4,
        B5 = 5,
        B6 = 6,
        B7 = 7,
        B8 = 8,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum RsaKeyComponent {
        Mod = RSA_COMP_MOD,
        PubExp = RSA_COMP_PUB_EXP,
        PrivExp = RSA_COMP_PRIV_EXP,
        P = RSA_COMP_P,
        Q = RSA_COMP_Q,
        Dp = RSA_COMP_DP,
        Dq = RSA_COMP_DQ,
        InvQ = RSA_COMP_INVQ,
        Na = RSA_COMP_NA,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum AttestationAlgo {
        // NOT SUPPORTED
        // ECdsaPlain = SIG_ECDSA_PLAIN,
        ECdsaSha = SIG_ECDSA_SHA,
        ECdsaSha224 = SIG_ECDSA_SHA_224,
        ECdsaSha256 = SIG_ECDSA_SHA_256,
        ECdsaSha384 = SIG_ECDSA_SHA_384,
        ECdsaSha512 = SIG_ECDSA_SHA_512,
        RsaSha1Pkcs1Pss = RSA_SHA1_PKCS1_PSS,
        RsaSha224Pkcs1Pss = RSA_SHA224_PKCS1_PSS,
        RsaSha256Pkcs1Pss = RSA_SHA256_PKCS1_PSS,
        RsaSha384Pkcs1Pss = RSA_SHA384_PKCS1_PSS,
        RsaSha512Pkcs1Pss = RSA_SHA512_PKCS1_PSS,
        RsaSha1Pkcs1 = RSA_SHA1_PKCS1,
        RsaSha224Pkcs1 = RSA_SHA_224_PKCS1,
        RsaSha256Pkcs1 = RSA_SHA_256_PKCS1,
        RsaSha384Pkcs1 = RSA_SHA_384_PKCS1,
        RsaSha512Pkcs1 = RSA_SHA_512_PKCS1,
    }
);
enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SecureObjectType {
        EcKeyPair = TYPE_EC_KEY_PAIR,
        EcPrivKey = TYPE_EC_PRIV_KEY,
        EcPubKey = TYPE_EC_PUB_KEY,
        RsaKeyPair = TYPE_RSA_KEY_PAIR,
        RsaKeyPairCrt = TYPE_RSA_KEY_PAIR_CRT,
        RsaPrivKey = TYPE_RSA_PRIV_KEY,
        RsaPrivKeyCrt = TYPE_RSA_PRIV_KEY_CRT,
        RsaPubKey = TYPE_RSA_PUB_KEY,
        AesKey = TYPE_AES_KEY,
        DesKey = TYPE_DES_KEY,
        BinaryFile = TYPE_BINARY_FILE,
        Userid = TYPE_USERID,
        Counter = TYPE_COUNTER,
        Pcr = TYPE_PCR,
        Curve = TYPE_CURVE,
        HmacKey = TYPE_HMAC_KEY,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SecureObjectFilter {
        EcKeyPair = TYPE_EC_KEY_PAIR,
        EcPrivKey = TYPE_EC_PRIV_KEY,
        EcPubKey = TYPE_EC_PUB_KEY,
        RsaKeyPair = TYPE_RSA_KEY_PAIR,
        RsaKeyPairCrt = TYPE_RSA_KEY_PAIR_CRT,
        RsaPrivKey = TYPE_RSA_PRIV_KEY,
        RsaPrivKeyCrt = TYPE_RSA_PRIV_KEY_CRT,
        RsaPubKey = TYPE_RSA_PUB_KEY,
        AesKey = TYPE_AES_KEY,
        DesKey = TYPE_DES_KEY,
        BinaryFile = TYPE_BINARY_FILE,
        Userid = TYPE_USERID,
        Counter = TYPE_COUNTER,
        Pcr = TYPE_PCR,
        Curve = TYPE_CURVE,
        HmacKey = TYPE_HMAC_KEY,
        All = 0xFF,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum MoreIndicator {
        More = MORE,
        NoMore = NO_MORE,
    }
);

impl MoreIndicator {
    pub fn is_more(&self) -> bool {
        matches!(self, Self::More)
    }
}

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Se05XResult {
        Success = RESULT_SUCCESS,
        Failure = RESULT_FAILURE,
    }
);

impl Se05XResult {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }
}

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum CryptoContext {
        Digest = CC_DIGEST,
        Cipher = CC_CIPHER,
        Signature = CC_SIGNATURE,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Digest {
        NoHash = DIGEST_NO_HASH,
        Sha = DIGEST_SHA,
        Sha224 = DIGEST_SHA224,
        Sha256 = DIGEST_SHA256,
        Sha384 = DIGEST_SHA384,
        Sha512 = DIGEST_SHA512,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum MacAlgo {
        HmacSha1 = HMAC_SHA1,
        HmacSha256 = HMAC_SHA256,
        HmacSha384 = HMAC_SHA384,
        HmacSha512 = HMAC_SHA512,
        DesMac4Iso9797M2 = DES_MAC4_ISO9797_M2,
        DesMac4Iso97971M2Alg3 = DES_MAC4_ISO9797_1_M2_ALG3,
        DesMac4Iso9797M1 = DES_MAC4_ISO9797_M1,
        DesMac4Iso97971M1Alg3 = DES_MAC4_ISO9797_1_M1_ALG3,
        DesMac8Iso9797M2 = DES_MAC8_ISO9797_M2,
        DesMac8Iso97971M2Alg3 = DES_MAC8_ISO9797_1_M2_ALG3,
        DesMac8Iso97971M1Alg3 = DES_MAC8_ISO9797_1_M1_ALG3,
        // DesMac8Iso97971M1Alg3 = DES_MAC8_ISO9797_1_M1_ALG3,
        Cmac128 = CMAC128,
        DesCmac8 = DES_CMAC8,
        AesCmac16 = AES_CMAC16,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum CipherMode {
        DesCbcNopad = DES_CBC_NOPAD,
        DesCbcIso9797M1 = DES_CBC_ISO9797_M1,
        DesCbcIso9797M2 = DES_CBC_ISO9797_M2,
        DesCbcPkcs5 = DES_CBC_PKCS5,
        DesEcbNopad = DES_ECB_NOPAD,
        DesEcbIso9797M1 = DES_ECB_ISO9797_M1,
        DesEcbIso9797M2 = DES_ECB_ISO9797_M2,
        DesEcbPkcs5 = DES_ECB_PKCS5,
        AesEcbNopad = AES_ECB_NOPAD,
        AesCbcNopad = AES_CBC_NOPAD,
        AesCbcIso9797M1 = AES_CBC_ISO9797_M1,
        AesCbcIso9797M2 = AES_CBC_ISO9797_M2,
        AesCbcPkcs5 = AES_CBC_PKCS5,
        AesCtr = AES_CTR,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum EcDsaSignatureAlgo {
        /// Not supported
        Plain = SIG_ECDSA_PLAIN,
        /// ECDSA with a SHA-1 digest as input.
        Sha = SIG_ECDSA_SHA,
        /// ECDSA with a SHA224 digest as input.
        Sha224 = SIG_ECDSA_SHA_224,
        /// ECDSA with a SHA256 digest as input.
        Sha256 = SIG_ECDSA_SHA_256,
        /// ECDSA with a SHA384 digest as input.
        Sha384 = SIG_ECDSA_SHA_384,
        /// ECDSA with a SHA512 digest as input.
        Sha512 = SIG_ECDSA_SHA_512,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum EdDsaSignatureAlgo {
        Pure = SIG_ED25519PURE,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum EcDaaSignatureAlgo {
        EcDaa = SIG_ECDAA,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Memory {
        Persistent = MEM_PERSISTENT,
        TransientReset = MEM_TRANSIENT_RESET,
        TransientDeselect = MEM_TRANSIENT_DESELECT,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum RsaSignatureAlgo {
        RsaSha1Pkcs1Pss = RSA_SHA1_PKCS1_PSS,
        RsaSha224Pkcs1Pss = RSA_SHA224_PKCS1_PSS,
        RsaSha256Pkcs1Pss = RSA_SHA256_PKCS1_PSS,
        RsaSha384Pkcs1Pss = RSA_SHA384_PKCS1_PSS,
        RsaSha512Pkcs1Pss = RSA_SHA512_PKCS1_PSS,
        RsaSha1Pkcs1 = RSA_SHA1_PKCS1,
        RsaSha224Pkcs1 = RSA_SHA_224_PKCS1,
        RsaSha256Pkcs1 = RSA_SHA_256_PKCS1,
        RsaSha384Pkcs1 = RSA_SHA_384_PKCS1,
        RsaSha512Pkcs1 = RSA_SHA_512_PKCS1,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum RsaEncryptionAlgo {
        NoPad = RSA_NO_PAD,
        Pkcs1 = RSA_PKCS1,
        Pkcs1Oaep = RSA_PKCS1_OAEP,
    }
);

enum_data!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SetIndicator {
        Set = SET,
        NotSet = NOT_SET,
    }
);

fn take_do_until<'data, E, T: TryFrom<&'data [u8], Error = E>>(
    tag: Tag,
    data: &'data [u8],
) -> Result<(T, &'data [u8]), Error>
where
    Error: From<E>,
{
    fn take_do_until_inner(tag: Tag, data: &[u8]) -> Result<(&[u8], &[u8]), Error> {
        let mut rem_inner = data;
        loop {
            let (read_tag, value, r) = take_data_object(rem_inner).ok_or(Error::Tlv)?;
            if read_tag == tag {
                return Ok((value, r));
            }
            rem_inner = r;
        }
    }

    let (value, rem) = take_do_until_inner(tag, data)?;
    Ok((value.try_into()?, rem))
}

/// `next` signals the tags that are expected after the read DO.
///
/// If the tag is  observed, then `take_opt_do_until` will return `None`
fn take_opt_do_until<'data, E, T: TryFrom<&'data [u8], Error = E>>(
    tag: Tag,
    next: &[Tag],
    data: &'data [u8],
) -> Result<(Option<T>, &'data [u8]), Error>
where
    Error: From<E>,
{
    fn take_opt_do_until_inner<'data>(
        tag: Tag,
        next: &[Tag],
        data: &'data [u8],
    ) -> Result<(Option<&'data [u8]>, &'data [u8]), Error> {
        let mut rem_inner = data;
        loop {
            let (read_tag, value, r) = take_data_object(rem_inner).ok_or(Error::Tlv)?;
            if read_tag == tag {
                return Ok((Some(value), r));
            } else if next.contains(&read_tag) {
                return Ok((None, rem_inner));
            }
            rem_inner = r;
        }
    }

    let (value, rem) = take_opt_do_until_inner(tag, next, data)?;
    Ok((value.map(TryInto::try_into).transpose()?, rem))
}

impl commands::ReadEcCurveListResponse<'_> {
    pub fn is_set(&self, curve: EcCurve) -> bool {
        let id: u8 = curve.into();
        if id >= 0x40 {
            return true;
        }

        self.ids.get(id as usize - 1) == Some(&SetIndicator::Set.into())
    }
}

#[cfg(test)]
mod tests {
    use super::{commands::CipherOneShotEncrypt, *};

    #[test]
    fn encrypt_length() {
        let key_id = ObjectId(hex!("03445566"));
        let plaintext_data = [2; 32 * 15];
        let iv = [0xFF; 16];
        let command = CipherOneShotEncrypt {
            key_id,
            mode: CipherMode::AesCbcPkcs5,
            plaintext: &plaintext_data,
            initialization_vector: Some(&iv),
        };
        assert!(command.len() < MAX_APDU_PAYLOAD_LENGTH);
    }
}
