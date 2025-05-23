// Copyright (C) 2023 Nitrokey GmbH
// SPDX-License-Identifier: LGPL-3.0-only

// Generated Automatically by `generate_commands.py DO NOT MODIFY MANUALLY

use super::policies::*;
use super::*;

// ************* CreateSession ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CreateSession {
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
}

impl DataSource for CreateSession {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataSource] = &[object_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_CREATE,
            __data,
            12,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CreateSession {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataStream<W>] = &[object_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_CREATE,
            __data,
            12,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateSessionResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub session_id: SessionId,
}

impl<'data> Se05XResponse<'data> for CreateSessionResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (session_id, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { session_id })
    }
}

impl<W: Writer> Se05XCommand<W> for CreateSession {
    type Response<'rdata> = CreateSessionResponse;
}

// ************* ExchangeSessionData ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ExchangeSessionData<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub session_policy: SessionPolicy,
    /// Serialized to remaining data
    pub c_mac: &'data [u8],
}

impl DataSource for ExchangeSessionData<'_> {
    fn len(&self) -> usize {
        let session_policy = &Tlv::new(TAG_1, self.session_policy);
        let c_mac = &self.c_mac;
        let __data: &[&dyn DataSource] = &[session_policy, c_mac];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_POLICY,
            __data,
            0,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ExchangeSessionData<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let session_policy = &Tlv::new(TAG_1, self.session_policy);
        let c_mac = &self.c_mac;
        let __data: &[&dyn DataStream<W>] = &[session_policy, c_mac];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_POLICY,
            __data,
            0,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExchangeSessionDataResponse<'data> {
    /// Parsed from remaining data
    pub r_mac: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ExchangeSessionDataResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let r_mac = rem;
        let _ = rem;
        Ok(Self { r_mac })
    }
}

impl<W: Writer> Se05XCommand<W> for ExchangeSessionData<'_> {
    type Response<'rdata> = ExchangeSessionDataResponse<'rdata>;
}

// ************* RefreshSession ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct RefreshSession {
    /// Serialized to TLV tag [`TAG_POLICY`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = policy_opt))))]
    pub policy: Option<SessionPolicy>,
}

impl DataSource for RefreshSession {
    fn len(&self) -> usize {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let __data: &[&dyn DataSource] = &[policy];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_REFRESH,
            __data,
            0,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for RefreshSession {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let __data: &[&dyn DataStream<W>] = &[policy];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_REFRESH,
            __data,
            0,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RefreshSessionResponse {}

impl<'data> Se05XResponse<'data> for RefreshSessionResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {})
    }
}

impl<W: Writer> Se05XCommand<W> for RefreshSession {
    type Response<'rdata> = RefreshSessionResponse;
}

// ************* CloseSession ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CloseSession {}

impl DataSource for CloseSession {
    fn len(&self) -> usize {
        let __data: &[&dyn DataSource] = &[];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_SESSION_CLOSE, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CloseSession {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let __data: &[&dyn DataStream<W>] = &[];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_SESSION_CLOSE, __data, 0);
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CloseSessionResponse {}

impl<'data> Se05XResponse<'data> for CloseSessionResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {})
    }
}

impl<W: Writer> Se05XCommand<W> for CloseSession {
    type Response<'rdata> = CloseSessionResponse;
}

// ************* VerifySessionUserId ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct VerifySessionUserId<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub user_id: &'data [u8],
}

impl DataSource for VerifySessionUserId<'_> {
    fn len(&self) -> usize {
        let user_id = &Tlv::new(TAG_1, self.user_id);
        let __data: &[&dyn DataSource] = &[user_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_USERID,
            __data,
            0,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for VerifySessionUserId<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let user_id = &Tlv::new(TAG_1, self.user_id);
        let __data: &[&dyn DataStream<W>] = &[user_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_USERID,
            __data,
            0,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifySessionUserIdResponse {}

impl<'data> Se05XResponse<'data> for VerifySessionUserIdResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {})
    }
}

impl<W: Writer> Se05XCommand<W> for VerifySessionUserId<'_> {
    type Response<'rdata> = VerifySessionUserIdResponse;
}

// ************* ScpInitializeUpdate ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ScpInitializeUpdate {
    /// Serialized to remaining data
    pub host_challenge: [u8; 8],
}

impl DataSource for ScpInitializeUpdate {
    fn len(&self) -> usize {
        let host_challenge = &self.host_challenge;
        let __data: &[&dyn DataSource] = &[host_challenge];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_INITIALIZE_UPDATE,
            P1_DEFAULT,
            P2_DEFAULT,
            __data,
            256,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ScpInitializeUpdate {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let host_challenge = &self.host_challenge;
        let __data: &[&dyn DataStream<W>] = &[host_challenge];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_INITIALIZE_UPDATE,
            P1_DEFAULT,
            P2_DEFAULT,
            __data,
            256,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScpInitializeUpdateResponse {
    /// Parsed from remaining data
    pub se05x_challenge: Se05xChallenge,
}

impl<'data> Se05XResponse<'data> for ScpInitializeUpdateResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let se05x_challenge = rem.try_into()?;
        let _ = rem;
        Ok(Self { se05x_challenge })
    }
}

impl<W: Writer> Se05XCommand<W> for ScpInitializeUpdate {
    type Response<'rdata> = ScpInitializeUpdateResponse;
}

// ************* ScpExternalAuthenticate ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ScpExternalAuthenticate {
    /// Serialized to remaining data
    pub host_cryptogram: [u8; 8],
    /// Serialized to remaining data
    pub mac: [u8; 8],
}

impl DataSource for ScpExternalAuthenticate {
    fn len(&self) -> usize {
        let host_cryptogram = &self.host_cryptogram;
        let mac = &self.mac;
        let __data: &[&dyn DataSource] = &[host_cryptogram, mac];
        let command = CommandBuilder::new(
            SM_CLA,
            INS_EXTERNAL_AUTHENTICATE,
            P1_DEFAULT,
            P2_DEFAULT,
            __data,
            0,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ScpExternalAuthenticate {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let host_cryptogram = &self.host_cryptogram;
        let mac = &self.mac;
        let __data: &[&dyn DataStream<W>] = &[host_cryptogram, mac];
        let command = CommandBuilder::new(
            SM_CLA,
            INS_EXTERNAL_AUTHENTICATE,
            P1_DEFAULT,
            P2_DEFAULT,
            __data,
            0,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScpExternalAuthenticateResponse {}

impl<'data> Se05XResponse<'data> for ScpExternalAuthenticateResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {})
    }
}

impl<W: Writer> Se05XCommand<W> for ScpExternalAuthenticate {
    type Response<'rdata> = ScpExternalAuthenticateResponse;
}

// ************* SetLockState ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct SetLockState {
    /// Serialized to TLV tag [`TAG_1`]()
    pub lock_indicator: TransientIndicator,
    /// Serialized to TLV tag [`TAG_2`]()
    pub lock_state: LockState,
}

impl DataSource for SetLockState {
    fn len(&self) -> usize {
        let lock_indicator = &Tlv::new(TAG_1, self.lock_indicator);
        let lock_state = &Tlv::new(TAG_2, self.lock_state);
        let __data: &[&dyn DataSource] = &[lock_indicator, lock_state];
        let command = CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_TRANSPORT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for SetLockState {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let lock_indicator = &Tlv::new(TAG_1, self.lock_indicator);
        let lock_state = &Tlv::new(TAG_2, self.lock_state);
        let __data: &[&dyn DataStream<W>] = &[lock_indicator, lock_state];
        let command = CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_TRANSPORT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for SetLockState {
    type Response<'rdata> = ();
}

// ************* WriteEcKey ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteEcKey<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    #[cfg_attr(feature = "builder", builder(default))]
    pub is_auth: bool,
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = key_type_opt))))]
    pub key_type: Option<P1KeyType>,
    /// Serialized to TLV tag [`TAG_POLICY`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = policy_opt))))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = max_attempts_opt))))]
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = curve_opt))))]
    pub curve: Option<EcCurve>,
    /// Serialized to TLV tag [`TAG_3`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = private_key_opt))))]
    pub private_key: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = public_key_opt))))]
    pub public_key: Option<&'data [u8]>,
}

impl DataSource for WriteEcKey<'_> {
    fn len(&self) -> usize {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let max_attempts = &self
            .max_attempts
            .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let curve = &self.curve.map(|data| Tlv::new(TAG_2, data));
        let private_key = &self.private_key.map(|data| Tlv::new(TAG_3, data));
        let public_key = &self.public_key.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataSource] = &[
            policy,
            max_attempts,
            object_id,
            curve,
            private_key,
            public_key,
        ];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };
        let p1: u8 = self.key_type.map(|v| v | P1_EC).unwrap_or(P1_EC);

        let command = CommandBuilder::new(NO_SM_CLA, ins, p1, P2_DEFAULT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for WriteEcKey<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let max_attempts = &self
            .max_attempts
            .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let curve = &self.curve.map(|data| Tlv::new(TAG_2, data));
        let private_key = &self.private_key.map(|data| Tlv::new(TAG_3, data));
        let public_key = &self.public_key.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataStream<W>] = &[
            policy,
            max_attempts,
            object_id,
            curve,
            private_key,
            public_key,
        ];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };
        let p1: u8 = self.key_type.map(|v| v | P1_EC).unwrap_or(P1_EC);

        let command = CommandBuilder::new(NO_SM_CLA, ins, p1, P2_DEFAULT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for WriteEcKey<'_> {
    type Response<'rdata> = ();
}

// ************* WriteRsaKey ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteRsaKey<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    #[cfg_attr(feature = "builder", builder(default))]
    pub is_auth: bool,
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = key_type_opt))))]
    pub key_type: Option<P1KeyType>,
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = key_format_opt))))]
    pub key_format: Option<RsaFormat>,
    /// Serialized to TLV tag [`TAG_POLICY`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = policy_opt))))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = max_attempts_opt))))]
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = key_size_opt))))]
    pub key_size: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = p_opt))))]
    pub p: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = q_opt))))]
    pub q: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_5`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = dp_opt))))]
    pub dp: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_6`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = dq_opt))))]
    pub dq: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_7`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = inv_q_opt))))]
    pub inv_q: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_8`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = e_opt))))]
    pub e: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_9`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = d_opt))))]
    pub d: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_10`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = n_opt))))]
    pub n: Option<&'data [u8]>,
}

impl DataSource for WriteRsaKey<'_> {
    fn len(&self) -> usize {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let max_attempts = &self
            .max_attempts
            .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let key_size = &self.key_size.map(|data| Tlv::new(TAG_2, data));
        let p = &self.p.map(|data| Tlv::new(TAG_3, data));
        let q = &self.q.map(|data| Tlv::new(TAG_4, data));
        let dp = &self.dp.map(|data| Tlv::new(TAG_5, data));
        let dq = &self.dq.map(|data| Tlv::new(TAG_6, data));
        let inv_q = &self.inv_q.map(|data| Tlv::new(TAG_7, data));
        let e = &self.e.map(|data| Tlv::new(TAG_8, data));
        let d = &self.d.map(|data| Tlv::new(TAG_9, data));
        let n = &self.n.map(|data| Tlv::new(TAG_10, data));
        let __data: &[&dyn DataSource] = &[
            policy,
            max_attempts,
            object_id,
            key_size,
            p,
            q,
            dp,
            dq,
            inv_q,
            e,
            d,
            n,
        ];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };
        let p1: u8 = self.key_type.map(|v| v | P1_RSA).unwrap_or(P1_RSA);
        let p2: u8 = self
            .key_format
            .map(|v| v | P2_DEFAULT)
            .unwrap_or(P2_DEFAULT);

        let command = CommandBuilder::new(NO_SM_CLA, ins, p1, p2, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for WriteRsaKey<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let max_attempts = &self
            .max_attempts
            .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let key_size = &self.key_size.map(|data| Tlv::new(TAG_2, data));
        let p = &self.p.map(|data| Tlv::new(TAG_3, data));
        let q = &self.q.map(|data| Tlv::new(TAG_4, data));
        let dp = &self.dp.map(|data| Tlv::new(TAG_5, data));
        let dq = &self.dq.map(|data| Tlv::new(TAG_6, data));
        let inv_q = &self.inv_q.map(|data| Tlv::new(TAG_7, data));
        let e = &self.e.map(|data| Tlv::new(TAG_8, data));
        let d = &self.d.map(|data| Tlv::new(TAG_9, data));
        let n = &self.n.map(|data| Tlv::new(TAG_10, data));
        let __data: &[&dyn DataStream<W>] = &[
            policy,
            max_attempts,
            object_id,
            key_size,
            p,
            q,
            dp,
            dq,
            inv_q,
            e,
            d,
            n,
        ];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };
        let p1: u8 = self.key_type.map(|v| v | P1_RSA).unwrap_or(P1_RSA);
        let p2: u8 = self
            .key_format
            .map(|v| v | P2_DEFAULT)
            .unwrap_or(P2_DEFAULT);

        let command = CommandBuilder::new(NO_SM_CLA, ins, p1, p2, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for WriteRsaKey<'_> {
    type Response<'rdata> = ();
}

// ************* GenRsaKey ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GenRsaKey<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    #[cfg_attr(feature = "builder", builder(default))]
    pub is_auth: bool,
    /// Serialized to TLV tag [`TAG_POLICY`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = policy_opt))))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = max_attempts_opt))))]
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = key_size_opt))))]
    pub key_size: Option<Be<u16>>,
}

impl DataSource for GenRsaKey<'_> {
    fn len(&self) -> usize {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let max_attempts = &self
            .max_attempts
            .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let key_size = &self.key_size.map(|data| Tlv::new(TAG_2, data));
        let __data: &[&dyn DataSource] = &[policy, max_attempts, object_id, key_size];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };

        let command = CommandBuilder::new(NO_SM_CLA, ins, P1_RSA | P1_KEY_PAIR, P2_RAW, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for GenRsaKey<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let max_attempts = &self
            .max_attempts
            .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let key_size = &self.key_size.map(|data| Tlv::new(TAG_2, data));
        let __data: &[&dyn DataStream<W>] = &[policy, max_attempts, object_id, key_size];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };

        let command = CommandBuilder::new(NO_SM_CLA, ins, P1_RSA | P1_KEY_PAIR, P2_RAW, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for GenRsaKey<'_> {
    type Response<'rdata> = ();
}

// ************* WriteSymmKey ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteSymmKey<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    #[cfg_attr(feature = "builder", builder(default))]
    pub is_auth: bool,
    pub key_type: SymmKeyType,
    /// Serialized to TLV tag [`TAG_POLICY`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = policy_opt))))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = max_attempts_opt))))]
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = kek_id_opt))))]
    pub kek_id: Option<ObjectId>,
    /// Serialized to TLV tag [`TAG_3`]()
    pub value: &'data [u8],
}

impl DataSource for WriteSymmKey<'_> {
    fn len(&self) -> usize {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let max_attempts = &self
            .max_attempts
            .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let kek_id = &self.kek_id.map(|data| Tlv::new(TAG_2, data));
        let value = &Tlv::new(TAG_3, self.value);
        let __data: &[&dyn DataSource] = &[policy, max_attempts, object_id, kek_id, value];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };
        let p1: u8 = self.key_type.into();

        let command = CommandBuilder::new(NO_SM_CLA, ins, p1, P2_DEFAULT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for WriteSymmKey<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let max_attempts = &self
            .max_attempts
            .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let kek_id = &self.kek_id.map(|data| Tlv::new(TAG_2, data));
        let value = &Tlv::new(TAG_3, self.value);
        let __data: &[&dyn DataStream<W>] = &[policy, max_attempts, object_id, kek_id, value];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };
        let p1: u8 = self.key_type.into();

        let command = CommandBuilder::new(NO_SM_CLA, ins, p1, P2_DEFAULT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for WriteSymmKey<'_> {
    type Response<'rdata> = ();
}

// ************* WriteBinary ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteBinary<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_POLICY`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = policy_opt))))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = offset_opt))))]
    pub offset: Option<Be<u16>>,
    /// Only when the object does not yet exists
    ///
    /// Serialized to TLV tag [`TAG_3`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = file_length_opt))))]
    pub file_length: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = data_opt))))]
    pub data: Option<&'data [u8]>,
}

impl DataSource for WriteBinary<'_> {
    fn len(&self) -> usize {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let offset = &self.offset.map(|data| Tlv::new(TAG_2, data));
        let file_length = &self.file_length.map(|data| Tlv::new(TAG_3, data));
        let data = &self.data.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataSource] = &[policy, object_id, offset, file_length, data];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

        let command = CommandBuilder::new(NO_SM_CLA, ins, P1_BINARY, P2_DEFAULT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for WriteBinary<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let offset = &self.offset.map(|data| Tlv::new(TAG_2, data));
        let file_length = &self.file_length.map(|data| Tlv::new(TAG_3, data));
        let data = &self.data.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataStream<W>] = &[policy, object_id, offset, file_length, data];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

        let command = CommandBuilder::new(NO_SM_CLA, ins, P1_BINARY, P2_DEFAULT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for WriteBinary<'_> {
    type Response<'rdata> = ();
}

// ************* WriteUserId ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteUserId<'data> {
    /// Serialized to TLV tag [`TAG_POLICY`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = policy_opt))))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = max_attempts_opt))))]
    pub max_attempts: Option<Be<u8>>,
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub data: &'data [u8],
}

impl DataSource for WriteUserId<'_> {
    fn len(&self) -> usize {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let max_attempts = &self
            .max_attempts
            .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let data = &Tlv::new(TAG_2, self.data);
        let __data: &[&dyn DataSource] = &[policy, max_attempts, object_id, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_WRITE | INS_AUTH_OBJECT,
            P1_USERID,
            P2_DEFAULT,
            __data,
            0,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for WriteUserId<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let max_attempts = &self
            .max_attempts
            .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let data = &Tlv::new(TAG_2, self.data);
        let __data: &[&dyn DataStream<W>] = &[policy, max_attempts, object_id, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_WRITE | INS_AUTH_OBJECT,
            P1_USERID,
            P2_DEFAULT,
            __data,
            0,
        );
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for WriteUserId<'_> {
    type Response<'rdata> = ();
}

// ************* WriteCounter ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteCounter<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_POLICY`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = policy_opt))))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = data_opt))))]
    pub data: Option<CounterSize>,
    /// Serialized to TLV tag [`TAG_3`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = value_opt))))]
    pub value: Option<Be<u64>>,
}

impl DataSource for WriteCounter<'_> {
    fn len(&self) -> usize {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let data = &self.data.map(|data| Tlv::new(TAG_2, data));
        let value = &self.value.map(|data| Tlv::new(TAG_3, data));
        let __data: &[&dyn DataSource] = &[policy, object_id, data, value];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

        let command = CommandBuilder::new(NO_SM_CLA, ins, P1_COUNTER, P2_DEFAULT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for WriteCounter<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let data = &self.data.map(|data| Tlv::new(TAG_2, data));
        let value = &self.value.map(|data| Tlv::new(TAG_3, data));
        let __data: &[&dyn DataStream<W>] = &[policy, object_id, data, value];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

        let command = CommandBuilder::new(NO_SM_CLA, ins, P1_COUNTER, P2_DEFAULT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for WriteCounter<'_> {
    type Response<'rdata> = ();
}

// ************* WritePcr ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WritePcr<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_POLICY`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = policy_opt))))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = initial_value_opt))))]
    pub initial_value: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_3`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = extend_opt))))]
    pub extend: Option<&'data [u8]>,
}

impl DataSource for WritePcr<'_> {
    fn len(&self) -> usize {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let initial_value = &self.initial_value.map(|data| Tlv::new(TAG_2, data));
        let extend = &self.extend.map(|data| Tlv::new(TAG_3, data));
        let __data: &[&dyn DataSource] = &[policy, object_id, initial_value, extend];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

        let command = CommandBuilder::new(NO_SM_CLA, ins, P1_PCR, P2_DEFAULT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for WritePcr<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let policy = &self.policy.map(|data| Tlv::new(TAG_POLICY, data));
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let initial_value = &self.initial_value.map(|data| Tlv::new(TAG_2, data));
        let extend = &self.extend.map(|data| Tlv::new(TAG_3, data));
        let __data: &[&dyn DataStream<W>] = &[policy, object_id, initial_value, extend];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

        let command = CommandBuilder::new(NO_SM_CLA, ins, P1_PCR, P2_DEFAULT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for WritePcr<'_> {
    type Response<'rdata> = ();
}

// ************* ImportObject ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ImportObject<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Unlike [`ExportObject::rsa_key_component`][], use None if not importing an RSA key
    ///
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = rsa_key_component_opt))))]
    pub rsa_key_component: Option<RsaKeyComponent>,
    /// Serialized to TLV tag [`TAG_3`]()
    pub serialized_object: &'data [u8],
}

impl DataSource for ImportObject<'_> {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let rsa_key_component = &self.rsa_key_component.map(|data| Tlv::new(TAG_2, data));
        let serialized_object = &Tlv::new(TAG_3, self.serialized_object);
        let __data: &[&dyn DataSource] = &[object_id, rsa_key_component, serialized_object];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

        let command = CommandBuilder::new(NO_SM_CLA, ins, P1_DEFAULT, P2_IMPORT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ImportObject<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let rsa_key_component = &self.rsa_key_component.map(|data| Tlv::new(TAG_2, data));
        let serialized_object = &Tlv::new(TAG_3, self.serialized_object);
        let __data: &[&dyn DataStream<W>] = &[object_id, rsa_key_component, serialized_object];
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

        let command = CommandBuilder::new(NO_SM_CLA, ins, P1_DEFAULT, P2_IMPORT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for ImportObject<'_> {
    type Response<'rdata> = ();
}

// ************* ReadObject ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadObject {
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = offset_opt))))]
    pub offset: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = length_opt))))]
    pub length: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = rsa_key_component_opt))))]
    pub rsa_key_component: Option<RsaKeyComponent>,
}

impl DataSource for ReadObject {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let offset = &self.offset.map(|data| Tlv::new(TAG_2, data));
        let length = &self.length.map(|data| Tlv::new(TAG_3, data));
        let rsa_key_component = &self.rsa_key_component.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataSource] = &[object_id, offset, length, rsa_key_component];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_DEFAULT,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ReadObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let offset = &self.offset.map(|data| Tlv::new(TAG_2, data));
        let length = &self.length.map(|data| Tlv::new(TAG_3, data));
        let rsa_key_component = &self.rsa_key_component.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataStream<W>] = &[object_id, offset, length, rsa_key_component];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_DEFAULT,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadObjectResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ReadObjectResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { data })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadObject {
    type Response<'rdata> = ReadObjectResponse<'rdata>;
}

// ************* ReadAttestObject ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadAttestObject<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = offset_opt))))]
    pub offset: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = length_opt))))]
    pub length: Option<Be<u16>>,
    /// Either [`Mod`][RsaKeyComponent::Mod] or [`PubExp`][RsaKeyComponent::PubExp]
    ///
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = rsa_key_component_opt))))]
    pub rsa_key_component: Option<RsaKeyComponent>,
    /// Serialized to TLV tag [`TAG_5`]()
    pub attestation_object: ObjectId,
    /// Serialized to TLV tag [`TAG_6`]()
    pub attestation_algo: AttestationAlgo,
    /// Serialized to TLV tag [`TAG_7`]()
    pub freshness_random: &'data [u8; 16],
}

impl DataSource for ReadAttestObject<'_> {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let offset = &self.offset.map(|data| Tlv::new(TAG_2, data));
        let length = &self.length.map(|data| Tlv::new(TAG_3, data));
        let rsa_key_component = &self.rsa_key_component.map(|data| Tlv::new(TAG_4, data));
        let attestation_object = &Tlv::new(TAG_5, self.attestation_object);
        let attestation_algo = &Tlv::new(TAG_6, self.attestation_algo);
        let freshness_random = &Tlv::new(TAG_7, self.freshness_random);
        let __data: &[&dyn DataSource] = &[
            object_id,
            offset,
            length,
            rsa_key_component,
            attestation_object,
            attestation_algo,
            freshness_random,
        ];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ | INS_ATTEST,
            P1_DEFAULT,
            P2_DEFAULT,
            __data,
            0,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ReadAttestObject<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let offset = &self.offset.map(|data| Tlv::new(TAG_2, data));
        let length = &self.length.map(|data| Tlv::new(TAG_3, data));
        let rsa_key_component = &self.rsa_key_component.map(|data| Tlv::new(TAG_4, data));
        let attestation_object = &Tlv::new(TAG_5, self.attestation_object);
        let attestation_algo = &Tlv::new(TAG_6, self.attestation_algo);
        let freshness_random = &Tlv::new(TAG_7, self.freshness_random);
        let __data: &[&dyn DataStream<W>] = &[
            object_id,
            offset,
            length,
            rsa_key_component,
            attestation_object,
            attestation_algo,
            freshness_random,
        ];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ | INS_ATTEST,
            P1_DEFAULT,
            P2_DEFAULT,
            __data,
            0,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadAttestObjectResponse<'data> {
    /// Is None when the object is a private key
    ///
    /// Parsed from TLV tag [`TAG_1`]()
    pub data: Option<&'data [u8]>,
    /// Parsed from TLV tag [`TAG_2`]()
    pub attributes: ObjectAttributes,
    /// Parsed from TLV tag [`TAG_3`]()
    pub timestamp: &'data [u8; 12],
    /// Parsed from TLV tag [`TAG_4`]()
    pub freshness_random: &'data [u8; 16],
    /// Parsed from TLV tag [`TAG_5`]()
    pub chip_unique_id: &'data [u8; 18],
    /// Parsed from TLV tag [`TAG_6`]()
    pub signature: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ReadAttestObjectResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) =
            take_opt_do_until(TAG_1, &[TAG_1, TAG_2, TAG_3, TAG_4, TAG_5, TAG_6], rem)?;
        let (attributes, rem) = take_do_until(TAG_2, rem)?;
        let (timestamp, rem) = take_do_until(TAG_3, rem)?;
        let (freshness_random, rem) = take_do_until(TAG_4, rem)?;
        let (chip_unique_id, rem) = take_do_until(TAG_5, rem)?;
        let (signature, rem) = take_do_until(TAG_6, rem)?;
        let _ = rem;
        Ok(Self {
            data,
            attributes,
            timestamp,
            freshness_random,
            chip_unique_id,
            signature,
        })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadAttestObject<'_> {
    type Response<'rdata> = ReadAttestObjectResponse<'rdata>;
}

// ************* ReadAttributes ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadAttributes<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = offset_opt))))]
    pub offset: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = length_opt))))]
    pub length: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = rsa_key_component_opt))))]
    pub rsa_key_component: Option<&'data [u8]>,
}

impl DataSource for ReadAttributes<'_> {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let offset = &self.offset.map(|data| Tlv::new(TAG_2, data));
        let length = &self.length.map(|data| Tlv::new(TAG_3, data));
        let rsa_key_component = &self.rsa_key_component.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataSource] = &[object_id, offset, length, rsa_key_component];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_ATTRIBUTES, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ReadAttributes<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let offset = &self.offset.map(|data| Tlv::new(TAG_2, data));
        let length = &self.length.map(|data| Tlv::new(TAG_3, data));
        let rsa_key_component = &self.rsa_key_component.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataStream<W>] = &[object_id, offset, length, rsa_key_component];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_ATTRIBUTES, __data, 0);
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadAttributesResponse {
    /// Parsed from TLV tag [`TAG_2`]()
    pub attributes: ObjectAttributes,
}

impl<'data> Se05XResponse<'data> for ReadAttributesResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (attributes, rem) = take_do_until(TAG_2, rem)?;
        let _ = rem;
        Ok(Self { attributes })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadAttributes<'_> {
    type Response<'rdata> = ReadAttributesResponse;
}

// ************* ReadAttributesAttest ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadAttributesAttest<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = offset_opt))))]
    pub offset: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = length_opt))))]
    pub length: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = rsa_key_component_opt))))]
    pub rsa_key_component: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_5`]()
    pub attestation_object: ObjectId,
    /// Serialized to TLV tag [`TAG_6`]()
    pub attestation_algo: AttestationAlgo,
    /// Serialized to TLV tag [`TAG_7`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = freshness_random_opt))))]
    pub freshness_random: Option<&'data [u8; 16]>,
}

impl DataSource for ReadAttributesAttest<'_> {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let offset = &self.offset.map(|data| Tlv::new(TAG_2, data));
        let length = &self.length.map(|data| Tlv::new(TAG_3, data));
        let rsa_key_component = &self.rsa_key_component.map(|data| Tlv::new(TAG_4, data));
        let attestation_object = &Tlv::new(TAG_5, self.attestation_object);
        let attestation_algo = &Tlv::new(TAG_6, self.attestation_algo);
        let freshness_random = &self.freshness_random.map(|data| Tlv::new(TAG_7, data));
        let __data: &[&dyn DataSource] = &[
            object_id,
            offset,
            length,
            rsa_key_component,
            attestation_object,
            attestation_algo,
            freshness_random,
        ];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ | INS_ATTEST,
            P1_DEFAULT,
            P2_ATTRIBUTES,
            __data,
            0,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ReadAttributesAttest<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let offset = &self.offset.map(|data| Tlv::new(TAG_2, data));
        let length = &self.length.map(|data| Tlv::new(TAG_3, data));
        let rsa_key_component = &self.rsa_key_component.map(|data| Tlv::new(TAG_4, data));
        let attestation_object = &Tlv::new(TAG_5, self.attestation_object);
        let attestation_algo = &Tlv::new(TAG_6, self.attestation_algo);
        let freshness_random = &self.freshness_random.map(|data| Tlv::new(TAG_7, data));
        let __data: &[&dyn DataStream<W>] = &[
            object_id,
            offset,
            length,
            rsa_key_component,
            attestation_object,
            attestation_algo,
            freshness_random,
        ];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ | INS_ATTEST,
            P1_DEFAULT,
            P2_ATTRIBUTES,
            __data,
            0,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadAttributesAttestResponse<'data> {
    /// Parsed from TLV tag [`TAG_2`]()
    pub attributes: ObjectAttributes,
    /// Parsed from TLV tag [`TAG_3`]()
    pub timestamp: &'data [u8; 12],
    /// Parsed from TLV tag [`TAG_4`]()
    pub freshness_random: &'data [u8; 16],
    /// Parsed from TLV tag [`TAG_5`]()
    pub chip_unique_id: &'data [u8; 18],
    /// Parsed from TLV tag [`TAG_6`]()
    pub signature: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ReadAttributesAttestResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (attributes, rem) = take_do_until(TAG_2, rem)?;
        let (timestamp, rem) = take_do_until(TAG_3, rem)?;
        let (freshness_random, rem) = take_do_until(TAG_4, rem)?;
        let (chip_unique_id, rem) = take_do_until(TAG_5, rem)?;
        let (signature, rem) = take_do_until(TAG_6, rem)?;
        let _ = rem;
        Ok(Self {
            attributes,
            timestamp,
            freshness_random,
            chip_unique_id,
            signature,
        })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadAttributesAttest<'_> {
    type Response<'rdata> = ReadAttributesAttestResponse<'rdata>;
}

// ************* ExportObject ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ExportObject {
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
    /// Always present. Use [`RsaKeyComponent::Na`][] if not exporting an RSA key. It is the default value with the builder API
    ///
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default=RsaKeyComponent::Na))]
    pub rsa_key_component: RsaKeyComponent,
}

impl DataSource for ExportObject {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let rsa_key_component = &Tlv::new(TAG_2, self.rsa_key_component);
        let __data: &[&dyn DataSource] = &[object_id, rsa_key_component];
        let command = CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_EXPORT, __data, 256)
            .force_extended();
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ExportObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let rsa_key_component = &Tlv::new(TAG_2, self.rsa_key_component);
        let __data: &[&dyn DataStream<W>] = &[object_id, rsa_key_component];
        let command = CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_EXPORT, __data, 256)
            .force_extended();
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportObjectResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ExportObjectResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { data })
    }
}

impl<W: Writer> Se05XCommand<W> for ExportObject {
    type Response<'rdata> = ExportObjectResponse<'rdata>;
}

// ************* ReadType ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadType {
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
}

impl DataSource for ReadType {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataSource] = &[object_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_TYPE,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ReadType {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataStream<W>] = &[object_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_TYPE,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadTypeResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub ty: SecureObjectType,
    /// Parsed from TLV tag [`TAG_2`]()
    pub transient_indicator: TransientIndicator,
}

impl<'data> Se05XResponse<'data> for ReadTypeResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (ty, rem) = take_do_until(TAG_1, rem)?;
        let (transient_indicator, rem) = take_do_until(TAG_2, rem)?;
        let _ = rem;
        Ok(Self {
            ty,
            transient_indicator,
        })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadType {
    type Response<'rdata> = ReadTypeResponse;
}

// ************* ReadSize ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadSize {
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
}

impl DataSource for ReadSize {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataSource] = &[object_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_SIZE,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ReadSize {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataStream<W>] = &[object_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_SIZE,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadSizeResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub size: Be<u64>,
}

impl<'data> Se05XResponse<'data> for ReadSizeResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (size, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { size })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadSize {
    type Response<'rdata> = ReadSizeResponse;
}

// ************* ReadIdList ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadIdList {
    /// Serialized to TLV tag [`TAG_1`]()
    pub offset: Be<u16>,
    /// Serialized to TLV tag [`TAG_2`]()
    pub filter: SecureObjectFilter,
}

impl DataSource for ReadIdList {
    fn len(&self) -> usize {
        let offset = &Tlv::new(TAG_1, self.offset);
        let filter = &Tlv::new(TAG_2, self.filter);
        let __data: &[&dyn DataSource] = &[offset, filter];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_LIST,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ReadIdList {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let offset = &Tlv::new(TAG_1, self.offset);
        let filter = &Tlv::new(TAG_2, self.filter);
        let __data: &[&dyn DataStream<W>] = &[offset, filter];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_LIST,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadIdListResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub more: MoreIndicator,
    /// Parsed from TLV tag [`TAG_2`]()
    pub ids: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ReadIdListResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (more, rem) = take_do_until(TAG_1, rem)?;
        let (ids, rem) = take_do_until(TAG_2, rem)?;
        let _ = rem;
        Ok(Self { more, ids })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadIdList {
    type Response<'rdata> = ReadIdListResponse<'rdata>;
}

// ************* CheckObjectExists ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CheckObjectExists {
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
}

impl DataSource for CheckObjectExists {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataSource] = &[object_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_EXIST,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CheckObjectExists {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataStream<W>] = &[object_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_EXIST,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckObjectExistsResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for CheckObjectExistsResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (result, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { result })
    }
}

impl<W: Writer> Se05XCommand<W> for CheckObjectExists {
    type Response<'rdata> = CheckObjectExistsResponse;
}

// ************* DeleteSecureObject ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DeleteSecureObject {
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
}

impl DataSource for DeleteSecureObject {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataSource] = &[object_id];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_DELETE_OBJECT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for DeleteSecureObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataStream<W>] = &[object_id];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_DELETE_OBJECT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for DeleteSecureObject {
    type Response<'rdata> = ();
}

// ************* CreateEcCurve ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CreateEcCurve {
    /// Serialized to TLV tag [`TAG_1`]()
    pub curve: EcCurve,
}

impl DataSource for CreateEcCurve {
    fn len(&self) -> usize {
        let curve = &Tlv::new(TAG_1, self.curve);
        let __data: &[&dyn DataSource] = &[curve];
        let command = CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CURVE, P2_CREATE, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CreateEcCurve {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let curve = &Tlv::new(TAG_1, self.curve);
        let __data: &[&dyn DataStream<W>] = &[curve];
        let command = CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CURVE, P2_CREATE, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for CreateEcCurve {
    type Response<'rdata> = ();
}

// ************* SetEcCurveParam ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct SetEcCurveParam<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub curve: EcCurve,
    /// Serialized to TLV tag [`TAG_2`]()
    pub param: EcCurveParam,
    /// Serialized to TLV tag [`TAG_3`]()
    pub value: &'data [u8],
}

impl DataSource for SetEcCurveParam<'_> {
    fn len(&self) -> usize {
        let curve = &Tlv::new(TAG_1, self.curve);
        let param = &Tlv::new(TAG_2, self.param);
        let value = &Tlv::new(TAG_3, self.value);
        let __data: &[&dyn DataSource] = &[curve, param, value];
        let command = CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CURVE, P2_PARAM, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for SetEcCurveParam<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let curve = &Tlv::new(TAG_1, self.curve);
        let param = &Tlv::new(TAG_2, self.param);
        let value = &Tlv::new(TAG_3, self.value);
        let __data: &[&dyn DataStream<W>] = &[curve, param, value];
        let command = CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CURVE, P2_PARAM, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for SetEcCurveParam<'_> {
    type Response<'rdata> = ();
}

// ************* GetEcCurveId ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GetEcCurveId {
    /// Serialized to TLV tag [`TAG_1`]()
    pub object_id: ObjectId,
}

impl DataSource for GetEcCurveId {
    fn len(&self) -> usize {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataSource] = &[object_id];
        let command = CommandBuilder::new(NO_SM_CLA, INS_READ, P1_CURVE, P2_ID, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for GetEcCurveId {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let object_id = &Tlv::new(TAG_1, self.object_id);
        let __data: &[&dyn DataStream<W>] = &[object_id];
        let command = CommandBuilder::new(NO_SM_CLA, INS_READ, P1_CURVE, P2_ID, __data, 0);
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetEcCurveIdResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub curve: EcCurve,
}

impl<'data> Se05XResponse<'data> for GetEcCurveIdResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (curve, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { curve })
    }
}

impl<W: Writer> Se05XCommand<W> for GetEcCurveId {
    type Response<'rdata> = GetEcCurveIdResponse;
}

// ************* ReadEcCurveList ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadEcCurveList {}

impl DataSource for ReadEcCurveList {
    fn len(&self) -> usize {
        let __data: &[&dyn DataSource] = &[];
        let command = CommandBuilder::new(NO_SM_CLA, INS_READ, P1_CURVE, P2_LIST, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ReadEcCurveList {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let __data: &[&dyn DataStream<W>] = &[];
        let command = CommandBuilder::new(NO_SM_CLA, INS_READ, P1_CURVE, P2_LIST, __data, 0);
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadEcCurveListResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub ids: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ReadEcCurveListResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (ids, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { ids })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadEcCurveList {
    type Response<'rdata> = ReadEcCurveListResponse<'rdata>;
}

// ************* DeleteEcCurve ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DeleteEcCurve {
    /// Serialized to TLV tag [`TAG_1`]()
    pub curve: EcCurve,
}

impl DataSource for DeleteEcCurve {
    fn len(&self) -> usize {
        let curve = &Tlv::new(TAG_1, self.curve);
        let __data: &[&dyn DataSource] = &[curve];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_CURVE, P2_DELETE_OBJECT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for DeleteEcCurve {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let curve = &Tlv::new(TAG_1, self.curve);
        let __data: &[&dyn DataStream<W>] = &[curve];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_CURVE, P2_DELETE_OBJECT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for DeleteEcCurve {
    type Response<'rdata> = ();
}

// ************* CreateDigestObject ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CreateDigestObject {
    /// Serialized to TLV tag [`TAG_1`]()
    pub id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_3`]()
    pub subtype: Digest,
}

impl DataSource for CreateDigestObject {
    fn len(&self) -> usize {
        let id = &Tlv::new(TAG_1, self.id);
        let context = &Tlv::new(TAG_2, CryptoContext::Digest);
        let subtype = &Tlv::new(TAG_3, self.subtype);
        let __data: &[&dyn DataSource] = &[id, context, subtype];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CRYPTO_OBJ, P2_DEFAULT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CreateDigestObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let id = &Tlv::new(TAG_1, self.id);
        let context = &Tlv::new(TAG_2, CryptoContext::Digest);
        let subtype = &Tlv::new(TAG_3, self.subtype);
        let __data: &[&dyn DataStream<W>] = &[id, context, subtype];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CRYPTO_OBJ, P2_DEFAULT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for CreateDigestObject {
    type Response<'rdata> = ();
}

// ************* CreateCipherObject ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CreateCipherObject {
    /// Serialized to TLV tag [`TAG_1`]()
    pub id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_3`]()
    pub subtype: CipherMode,
}

impl DataSource for CreateCipherObject {
    fn len(&self) -> usize {
        let id = &Tlv::new(TAG_1, self.id);
        let context = &Tlv::new(TAG_2, CryptoContext::Cipher);
        let subtype = &Tlv::new(TAG_3, self.subtype);
        let __data: &[&dyn DataSource] = &[id, context, subtype];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CRYPTO_OBJ, P2_DEFAULT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CreateCipherObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let id = &Tlv::new(TAG_1, self.id);
        let context = &Tlv::new(TAG_2, CryptoContext::Cipher);
        let subtype = &Tlv::new(TAG_3, self.subtype);
        let __data: &[&dyn DataStream<W>] = &[id, context, subtype];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CRYPTO_OBJ, P2_DEFAULT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for CreateCipherObject {
    type Response<'rdata> = ();
}

// ************* CreateSignatureObject ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CreateSignatureObject {
    /// Serialized to TLV tag [`TAG_1`]()
    pub id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_3`]()
    pub subtype: MacAlgo,
}

impl DataSource for CreateSignatureObject {
    fn len(&self) -> usize {
        let id = &Tlv::new(TAG_1, self.id);
        let context = &Tlv::new(TAG_2, CryptoContext::Signature);
        let subtype = &Tlv::new(TAG_3, self.subtype);
        let __data: &[&dyn DataSource] = &[id, context, subtype];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CRYPTO_OBJ, P2_DEFAULT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CreateSignatureObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let id = &Tlv::new(TAG_1, self.id);
        let context = &Tlv::new(TAG_2, CryptoContext::Signature);
        let subtype = &Tlv::new(TAG_3, self.subtype);
        let __data: &[&dyn DataStream<W>] = &[id, context, subtype];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CRYPTO_OBJ, P2_DEFAULT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for CreateSignatureObject {
    type Response<'rdata> = ();
}

// ************* ReadCryptoObjList ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadCryptoObjList {}

impl DataSource for ReadCryptoObjList {
    fn len(&self) -> usize {
        let __data: &[&dyn DataSource] = &[];
        let command = CommandBuilder::new(NO_SM_CLA, INS_READ, P1_CRYPTO_OBJ, P2_LIST, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for ReadCryptoObjList {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let __data: &[&dyn DataStream<W>] = &[];
        let command = CommandBuilder::new(NO_SM_CLA, INS_READ, P1_CRYPTO_OBJ, P2_LIST, __data, 0);
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadCryptoObjListResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub list: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ReadCryptoObjListResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (list, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { list })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadCryptoObjList {
    type Response<'rdata> = ReadCryptoObjListResponse<'rdata>;
}

// ************* DeleteCryptoObj ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DeleteCryptoObj {
    /// Serialized to TLV tag [`TAG_1`]()
    pub id: CryptoObjectId,
}

impl DataSource for DeleteCryptoObj {
    fn len(&self) -> usize {
        let id = &Tlv::new(TAG_1, self.id);
        let __data: &[&dyn DataSource] = &[id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_CRYPTO_OBJ,
            P2_DELETE_OBJECT,
            __data,
            0,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for DeleteCryptoObj {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let id = &Tlv::new(TAG_1, self.id);
        let __data: &[&dyn DataStream<W>] = &[id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_CRYPTO_OBJ,
            P2_DELETE_OBJECT,
            __data,
            0,
        );
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for DeleteCryptoObj {
    type Response<'rdata> = ();
}

// ************* EcdsaSign ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EcdsaSign<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub algo: EcDsaSignatureAlgo,
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
}

impl DataSource for EcdsaSign<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataSource] = &[key_id, algo, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for EcdsaSign<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataStream<W>] = &[key_id, algo, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EcdsaSignResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub signature: &'data [u8],
}

impl<'data> Se05XResponse<'data> for EcdsaSignResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (signature, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { signature })
    }
}

impl<W: Writer> Se05XCommand<W> for EcdsaSign<'_> {
    type Response<'rdata> = EcdsaSignResponse<'rdata>;
}

// ************* EddsaSign ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EddsaSign<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
}

impl DataSource for EddsaSign<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, EdDsaSignatureAlgo::Pure);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataSource] = &[key_id, algo, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for EddsaSign<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, EdDsaSignatureAlgo::Pure);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataStream<W>] = &[key_id, algo, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EddsaSignResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub signature: &'data [u8],
}

impl<'data> Se05XResponse<'data> for EddsaSignResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (signature, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { signature })
    }
}

impl<W: Writer> Se05XCommand<W> for EddsaSign<'_> {
    type Response<'rdata> = EddsaSignResponse<'rdata>;
}

// ************* EcdaaSign ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EcdaaSign {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: [u8; 32],
    /// Serialized to TLV tag [`TAG_4`]()
    pub random_data: [u8; 32],
}

impl DataSource for EcdaaSign {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, EcDaaSignatureAlgo::EcDaa);
        let data = &Tlv::new(TAG_3, self.data);
        let random_data = &Tlv::new(TAG_4, self.random_data);
        let __data: &[&dyn DataSource] = &[key_id, algo, data, random_data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for EcdaaSign {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, EcDaaSignatureAlgo::EcDaa);
        let data = &Tlv::new(TAG_3, self.data);
        let random_data = &Tlv::new(TAG_4, self.random_data);
        let __data: &[&dyn DataStream<W>] = &[key_id, algo, data, random_data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EcdaaSignResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub signature: &'data [u8],
}

impl<'data> Se05XResponse<'data> for EcdaaSignResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (signature, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { signature })
    }
}

impl<W: Writer> Se05XCommand<W> for EcdaaSign {
    type Response<'rdata> = EcdaaSignResponse<'rdata>;
}

// ************* EcdsaVerify ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EcdsaVerify<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub algo: EcDsaSignatureAlgo,
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_5`]()
    pub signature: &'data [u8],
}

impl DataSource for EcdsaVerify<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let signature = &Tlv::new(TAG_5, self.signature);
        let __data: &[&dyn DataSource] = &[key_id, algo, data, signature];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_SIGNATURE, P2_VERIFY, __data, 3);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for EcdsaVerify<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let signature = &Tlv::new(TAG_5, self.signature);
        let __data: &[&dyn DataStream<W>] = &[key_id, algo, data, signature];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_SIGNATURE, P2_VERIFY, __data, 3);
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EcdsaVerifyResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for EcdsaVerifyResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (result, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { result })
    }
}

impl<W: Writer> Se05XCommand<W> for EcdsaVerify<'_> {
    type Response<'rdata> = EcdsaVerifyResponse;
}

// ************* EddsaVerify ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EddsaVerify<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_5`]()
    pub signature: &'data [u8],
}

impl DataSource for EddsaVerify<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, EdDsaSignatureAlgo::Pure);
        let data = &Tlv::new(TAG_3, self.data);
        let signature = &Tlv::new(TAG_5, self.signature);
        let __data: &[&dyn DataSource] = &[key_id, algo, data, signature];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_SIGNATURE, P2_VERIFY, __data, 3);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for EddsaVerify<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, EdDsaSignatureAlgo::Pure);
        let data = &Tlv::new(TAG_3, self.data);
        let signature = &Tlv::new(TAG_5, self.signature);
        let __data: &[&dyn DataStream<W>] = &[key_id, algo, data, signature];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_SIGNATURE, P2_VERIFY, __data, 3);
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EddsaVerifyResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for EddsaVerifyResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (result, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { result })
    }
}

impl<W: Writer> Se05XCommand<W> for EddsaVerify<'_> {
    type Response<'rdata> = EddsaVerifyResponse;
}

// ************* EcdhGenerateSharedSecret ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EcdhGenerateSharedSecret<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub public_key: &'data [u8],
}

impl DataSource for EcdhGenerateSharedSecret<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let public_key = &Tlv::new(TAG_2, self.public_key);
        let __data: &[&dyn DataSource] = &[key_id, public_key];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_EC,
            P2_DH,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for EcdhGenerateSharedSecret<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let public_key = &Tlv::new(TAG_2, self.public_key);
        let __data: &[&dyn DataStream<W>] = &[key_id, public_key];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_EC,
            P2_DH,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EcdhGenerateSharedSecretResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub shared_secret: &'data [u8],
}

impl<'data> Se05XResponse<'data> for EcdhGenerateSharedSecretResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (shared_secret, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { shared_secret })
    }
}

impl<W: Writer> Se05XCommand<W> for EcdhGenerateSharedSecret<'_> {
    type Response<'rdata> = EcdhGenerateSharedSecretResponse<'rdata>;
}

// ************* RsaSign ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct RsaSign<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub algo: RsaSignatureAlgo,
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
}

impl DataSource for RsaSign<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataSource] = &[key_id, algo, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for RsaSign<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataStream<W>] = &[key_id, algo, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RsaSignResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub signature: &'data [u8],
}

impl<'data> Se05XResponse<'data> for RsaSignResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (signature, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { signature })
    }
}

impl<W: Writer> Se05XCommand<W> for RsaSign<'_> {
    type Response<'rdata> = RsaSignResponse<'rdata>;
}

// ************* RsaVerify ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct RsaVerify<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub algo: RsaSignatureAlgo,
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_5`]()
    pub signature: &'data [u8],
}

impl DataSource for RsaVerify<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let signature = &Tlv::new(TAG_5, self.signature);
        let __data: &[&dyn DataSource] = &[key_id, algo, data, signature];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_SIGNATURE, P2_VERIFY, __data, 3);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for RsaVerify<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let signature = &Tlv::new(TAG_5, self.signature);
        let __data: &[&dyn DataStream<W>] = &[key_id, algo, data, signature];
        let command =
            CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_SIGNATURE, P2_VERIFY, __data, 3);
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RsaVerifyResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for RsaVerifyResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (result, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { result })
    }
}

impl<W: Writer> Se05XCommand<W> for RsaVerify<'_> {
    type Response<'rdata> = RsaVerifyResponse;
}

// ************* RsaEncrypt ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct RsaEncrypt<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub algo: RsaEncryptionAlgo,
    /// Serialized to TLV tag [`TAG_3`]()
    pub plaintext: &'data [u8],
}

impl DataSource for RsaEncrypt<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let plaintext = &Tlv::new(TAG_3, self.plaintext);
        let __data: &[&dyn DataSource] = &[key_id, algo, plaintext];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_RSA,
            P2_ENCRYPT_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for RsaEncrypt<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let plaintext = &Tlv::new(TAG_3, self.plaintext);
        let __data: &[&dyn DataStream<W>] = &[key_id, algo, plaintext];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_RSA,
            P2_ENCRYPT_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RsaEncryptResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub ciphertext: &'data [u8],
}

impl<'data> Se05XResponse<'data> for RsaEncryptResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (ciphertext, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { ciphertext })
    }
}

impl<W: Writer> Se05XCommand<W> for RsaEncrypt<'_> {
    type Response<'rdata> = RsaEncryptResponse<'rdata>;
}

// ************* RsaDecrypt ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct RsaDecrypt<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub algo: RsaEncryptionAlgo,
    /// Serialized to TLV tag [`TAG_3`]()
    pub ciphertext: &'data [u8],
}

impl DataSource for RsaDecrypt<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let ciphertext = &Tlv::new(TAG_3, self.ciphertext);
        let __data: &[&dyn DataSource] = &[key_id, algo, ciphertext];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_RSA,
            P2_DECRYPT_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for RsaDecrypt<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let ciphertext = &Tlv::new(TAG_3, self.ciphertext);
        let __data: &[&dyn DataStream<W>] = &[key_id, algo, ciphertext];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_RSA,
            P2_DECRYPT_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RsaDecryptResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub plaintext: &'data [u8],
}

impl<'data> Se05XResponse<'data> for RsaDecryptResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (plaintext, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { plaintext })
    }
}

impl<W: Writer> Se05XCommand<W> for RsaDecrypt<'_> {
    type Response<'rdata> = RsaDecryptResponse<'rdata>;
}

// ************* CipherEncryptInit ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherEncryptInit<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub cipher_id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = initialization_vector_opt))))]
    pub initialization_vector: Option<&'data [u8]>,
}

impl DataSource for CipherEncryptInit<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let cipher_id = &Tlv::new(TAG_2, self.cipher_id);
        let initialization_vector = &self.initialization_vector.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataSource] = &[key_id, cipher_id, initialization_vector];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_CIPHER, P2_ENCRYPT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CipherEncryptInit<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let cipher_id = &Tlv::new(TAG_2, self.cipher_id);
        let initialization_vector = &self.initialization_vector.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataStream<W>] = &[key_id, cipher_id, initialization_vector];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_CIPHER, P2_ENCRYPT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for CipherEncryptInit<'_> {
    type Response<'rdata> = ();
}

// ************* CipherDecryptInit ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherDecryptInit<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub cipher_id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = initialization_vector_opt))))]
    pub initialization_vector: Option<&'data [u8]>,
}

impl DataSource for CipherDecryptInit<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let cipher_id = &Tlv::new(TAG_2, self.cipher_id);
        let initialization_vector = &self.initialization_vector.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataSource] = &[key_id, cipher_id, initialization_vector];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_CIPHER, P2_DECRYPT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CipherDecryptInit<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let cipher_id = &Tlv::new(TAG_2, self.cipher_id);
        let initialization_vector = &self.initialization_vector.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataStream<W>] = &[key_id, cipher_id, initialization_vector];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_CIPHER, P2_DECRYPT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for CipherDecryptInit<'_> {
    type Response<'rdata> = ();
}

// ************* CipherUpdate ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherUpdate<'data> {
    /// Serialized to TLV tag [`TAG_2`]()
    pub cipher_id: CryptoObjectId,
    /// input data, can be either plaintext or ciphertext depending on whether cipher_decrypt_init or cipher_encrypt_init was used
    ///
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
}

impl DataSource for CipherUpdate<'_> {
    fn len(&self) -> usize {
        let cipher_id = &Tlv::new(TAG_2, self.cipher_id);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataSource] = &[cipher_id, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_UPDATE,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CipherUpdate<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let cipher_id = &Tlv::new(TAG_2, self.cipher_id);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataStream<W>] = &[cipher_id, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_UPDATE,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CipherUpdateResponse<'data> {
    /// output data
    ///
    /// Parsed from TLV tag [`TAG_1`]()
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for CipherUpdateResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { data })
    }
}

impl<W: Writer> Se05XCommand<W> for CipherUpdate<'_> {
    type Response<'rdata> = CipherUpdateResponse<'rdata>;
}

// ************* CipherFinal ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherFinal<'data> {
    /// Serialized to TLV tag [`TAG_2`]()
    pub cipher_id: CryptoObjectId,
    /// input data, can be either plaintext or ciphertext depending on whether cipher_decrypt_init or cipher_encrypt_init was used
    ///
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
}

impl DataSource for CipherFinal<'_> {
    fn len(&self) -> usize {
        let cipher_id = &Tlv::new(TAG_2, self.cipher_id);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataSource] = &[cipher_id, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_FINAL,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CipherFinal<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let cipher_id = &Tlv::new(TAG_2, self.cipher_id);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataStream<W>] = &[cipher_id, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_FINAL,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CipherFinalResponse<'data> {
    /// output data
    ///
    /// Parsed from TLV tag [`TAG_1`]()
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for CipherFinalResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { data })
    }
}

impl<W: Writer> Se05XCommand<W> for CipherFinal<'_> {
    type Response<'rdata> = CipherFinalResponse<'rdata>;
}

// ************* CipherOneShotEncrypt ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherOneShotEncrypt<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub mode: CipherMode,
    /// Serialized to TLV tag [`TAG_3`]()
    pub plaintext: &'data [u8],
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = initialization_vector_opt))))]
    pub initialization_vector: Option<&'data [u8]>,
}

impl DataSource for CipherOneShotEncrypt<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let mode = &Tlv::new(TAG_2, self.mode);
        let plaintext = &Tlv::new(TAG_3, self.plaintext);
        let initialization_vector = &self.initialization_vector.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataSource] = &[key_id, mode, plaintext, initialization_vector];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_ENCRYPT_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CipherOneShotEncrypt<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let mode = &Tlv::new(TAG_2, self.mode);
        let plaintext = &Tlv::new(TAG_3, self.plaintext);
        let initialization_vector = &self.initialization_vector.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataStream<W>] = &[key_id, mode, plaintext, initialization_vector];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_ENCRYPT_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CipherOneShotEncryptResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub ciphertext: &'data [u8],
}

impl<'data> Se05XResponse<'data> for CipherOneShotEncryptResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (ciphertext, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { ciphertext })
    }
}

impl<W: Writer> Se05XCommand<W> for CipherOneShotEncrypt<'_> {
    type Response<'rdata> = CipherOneShotEncryptResponse<'rdata>;
}

// ************* CipherOneShotDecrypt ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherOneShotDecrypt<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub mode: CipherMode,
    /// Serialized to TLV tag [`TAG_3`]()
    pub ciphertext: &'data [u8],
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = initialization_vector_opt))))]
    pub initialization_vector: Option<&'data [u8]>,
}

impl DataSource for CipherOneShotDecrypt<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let mode = &Tlv::new(TAG_2, self.mode);
        let ciphertext = &Tlv::new(TAG_3, self.ciphertext);
        let initialization_vector = &self.initialization_vector.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataSource] = &[key_id, mode, ciphertext, initialization_vector];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_DECRYPT_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for CipherOneShotDecrypt<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let mode = &Tlv::new(TAG_2, self.mode);
        let ciphertext = &Tlv::new(TAG_3, self.ciphertext);
        let initialization_vector = &self.initialization_vector.map(|data| Tlv::new(TAG_4, data));
        let __data: &[&dyn DataStream<W>] = &[key_id, mode, ciphertext, initialization_vector];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_DECRYPT_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CipherOneShotDecryptResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub plaintext: &'data [u8],
}

impl<'data> Se05XResponse<'data> for CipherOneShotDecryptResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (plaintext, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { plaintext })
    }
}

impl<W: Writer> Se05XCommand<W> for CipherOneShotDecrypt<'_> {
    type Response<'rdata> = CipherOneShotDecryptResponse<'rdata>;
}

// ************* MacGenerateInit ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacGenerateInit {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub mac_id: CryptoObjectId,
}

impl DataSource for MacGenerateInit {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let mac_id = &Tlv::new(TAG_2, self.mac_id);
        let __data: &[&dyn DataSource] = &[key_id, mac_id];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_MAC, P2_GENERATE, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for MacGenerateInit {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let mac_id = &Tlv::new(TAG_2, self.mac_id);
        let __data: &[&dyn DataStream<W>] = &[key_id, mac_id];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_MAC, P2_GENERATE, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for MacGenerateInit {
    type Response<'rdata> = ();
}

// ************* MacValidateInit ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacValidateInit {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub mac_id: CryptoObjectId,
}

impl DataSource for MacValidateInit {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let mac_id = &Tlv::new(TAG_2, self.mac_id);
        let __data: &[&dyn DataSource] = &[key_id, mac_id];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_MAC, P2_VALIDATE, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for MacValidateInit {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let mac_id = &Tlv::new(TAG_2, self.mac_id);
        let __data: &[&dyn DataStream<W>] = &[key_id, mac_id];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_MAC, P2_VALIDATE, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for MacValidateInit {
    type Response<'rdata> = ();
}

// ************* MacUpdate ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacUpdate<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_2`]()
    pub mac_id: CryptoObjectId,
}

impl DataSource for MacUpdate<'_> {
    fn len(&self) -> usize {
        let data = &Tlv::new(TAG_1, self.data);
        let mac_id = &Tlv::new(TAG_2, self.mac_id);
        let __data: &[&dyn DataSource] = &[data, mac_id];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_MAC, P2_UPDATE, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for MacUpdate<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let data = &Tlv::new(TAG_1, self.data);
        let mac_id = &Tlv::new(TAG_2, self.mac_id);
        let __data: &[&dyn DataStream<W>] = &[data, mac_id];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_MAC, P2_UPDATE, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for MacUpdate<'_> {
    type Response<'rdata> = ();
}

// ************* MacGenerateFinal ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacGenerateFinal<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_2`]()
    pub mac_id: CryptoObjectId,
}

impl DataSource for MacGenerateFinal<'_> {
    fn len(&self) -> usize {
        let data = &Tlv::new(TAG_1, self.data);
        let mac_id = &Tlv::new(TAG_2, self.mac_id);
        let __data: &[&dyn DataSource] = &[data, mac_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_FINAL,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for MacGenerateFinal<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let data = &Tlv::new(TAG_1, self.data);
        let mac_id = &Tlv::new(TAG_2, self.mac_id);
        let __data: &[&dyn DataStream<W>] = &[data, mac_id];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_FINAL,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MacGenerateFinalResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub tag: &'data [u8],
}

impl<'data> Se05XResponse<'data> for MacGenerateFinalResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (tag, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { tag })
    }
}

impl<W: Writer> Se05XCommand<W> for MacGenerateFinal<'_> {
    type Response<'rdata> = MacGenerateFinalResponse<'rdata>;
}

// ************* MacValidateFinal ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacValidateFinal<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_2`]()
    pub mac_id: CryptoObjectId,
    /// Tag to validate
    ///
    /// Serialized to TLV tag [`TAG_3`]()
    pub tag: &'data [u8],
}

impl DataSource for MacValidateFinal<'_> {
    fn len(&self) -> usize {
        let data = &Tlv::new(TAG_1, self.data);
        let mac_id = &Tlv::new(TAG_2, self.mac_id);
        let tag = &Tlv::new(TAG_3, self.tag);
        let __data: &[&dyn DataSource] = &[data, mac_id, tag];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_FINAL,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for MacValidateFinal<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let data = &Tlv::new(TAG_1, self.data);
        let mac_id = &Tlv::new(TAG_2, self.mac_id);
        let tag = &Tlv::new(TAG_3, self.tag);
        let __data: &[&dyn DataStream<W>] = &[data, mac_id, tag];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_FINAL,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MacValidateFinalResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for MacValidateFinalResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (result, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { result })
    }
}

impl<W: Writer> Se05XCommand<W> for MacValidateFinal<'_> {
    type Response<'rdata> = MacValidateFinalResponse;
}

// ************* MacOneShotGenerate ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacOneShotGenerate<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub algo: MacAlgo,
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
}

impl DataSource for MacOneShotGenerate<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataSource] = &[key_id, algo, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_GENERATE_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for MacOneShotGenerate<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataStream<W>] = &[key_id, algo, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_GENERATE_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MacOneShotGenerateResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub tag: &'data [u8],
}

impl<'data> Se05XResponse<'data> for MacOneShotGenerateResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (tag, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { tag })
    }
}

impl<W: Writer> Se05XCommand<W> for MacOneShotGenerate<'_> {
    type Response<'rdata> = MacOneShotGenerateResponse<'rdata>;
}

// ************* MacOneShotValidate ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacOneShotValidate<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub algo: MacAlgo,
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
    /// tag to validate
    ///
    /// Serialized to TLV tag [`TAG_5`]()
    pub tag: &'data [u8],
}

impl DataSource for MacOneShotValidate<'_> {
    fn len(&self) -> usize {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let tag = &Tlv::new(TAG_5, self.tag);
        let __data: &[&dyn DataSource] = &[key_id, algo, data, tag];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_VALIDATE_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for MacOneShotValidate<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let key_id = &Tlv::new(TAG_1, self.key_id);
        let algo = &Tlv::new(TAG_2, self.algo);
        let data = &Tlv::new(TAG_3, self.data);
        let tag = &Tlv::new(TAG_5, self.tag);
        let __data: &[&dyn DataStream<W>] = &[key_id, algo, data, tag];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_VALIDATE_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MacOneShotValidateResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for MacOneShotValidateResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (result, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { result })
    }
}

impl<W: Writer> Se05XCommand<W> for MacOneShotValidate<'_> {
    type Response<'rdata> = MacOneShotValidateResponse;
}

// ************* Hkdf ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct Hkdf<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub ikm: ObjectId,
    /// Serialized to TLV tag [`TAG_2`]()
    pub digest: Digest,
    /// up to 64 bytes
    ///
    /// Serialized to TLV tag [`TAG_3`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = salt_opt))))]
    pub salt: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_4`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = info_opt))))]
    pub info: Option<&'data [u8]>,
    /// Up to MAX_APDU_PAYLOAD_LENGTH (= 889)
    ///
    /// Serialized to TLV tag [`TAG_5`]()
    pub requested_len: Be<u16>,
}

impl DataSource for Hkdf<'_> {
    fn len(&self) -> usize {
        let ikm = &Tlv::new(TAG_1, self.ikm);
        let digest = &Tlv::new(TAG_2, self.digest);
        let salt = &self.salt.map(|data| Tlv::new(TAG_3, data));
        let info = &self.info.map(|data| Tlv::new(TAG_4, data));
        let requested_len = &Tlv::new(TAG_5, self.requested_len);
        let __data: &[&dyn DataSource] = &[ikm, digest, salt, info, requested_len];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_HKDF,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for Hkdf<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let ikm = &Tlv::new(TAG_1, self.ikm);
        let digest = &Tlv::new(TAG_2, self.digest);
        let salt = &self.salt.map(|data| Tlv::new(TAG_3, data));
        let info = &self.info.map(|data| Tlv::new(TAG_4, data));
        let requested_len = &Tlv::new(TAG_5, self.requested_len);
        let __data: &[&dyn DataStream<W>] = &[ikm, digest, salt, info, requested_len];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_HKDF,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HkdfResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for HkdfResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { data })
    }
}

impl<W: Writer> Se05XCommand<W> for Hkdf<'_> {
    type Response<'rdata> = HkdfResponse<'rdata>;
}

// ************* Pbkdf2 ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct Pbkdf2<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub password: ObjectId,
    /// up to 64 bytes
    ///
    /// Serialized to TLV tag [`TAG_2`]()
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option(fallback = salt_opt))))]
    pub salt: Option<&'data [u8]>,
    /// Up to 0x7FFF
    ///
    /// Serialized to TLV tag [`TAG_3`]()
    pub iterations: Be<u16>,
    /// Up to 512
    ///
    /// Serialized to TLV tag [`TAG_4`]()
    pub requested_len: Be<u16>,
}

impl DataSource for Pbkdf2<'_> {
    fn len(&self) -> usize {
        let password = &Tlv::new(TAG_1, self.password);
        let salt = &self.salt.map(|data| Tlv::new(TAG_2, data));
        let iterations = &Tlv::new(TAG_3, self.iterations);
        let requested_len = &Tlv::new(TAG_4, self.requested_len);
        let __data: &[&dyn DataSource] = &[password, salt, iterations, requested_len];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_PBKDF,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for Pbkdf2<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let password = &Tlv::new(TAG_1, self.password);
        let salt = &self.salt.map(|data| Tlv::new(TAG_2, data));
        let iterations = &Tlv::new(TAG_3, self.iterations);
        let requested_len = &Tlv::new(TAG_4, self.requested_len);
        let __data: &[&dyn DataStream<W>] = &[password, salt, iterations, requested_len];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_PBKDF,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pbkdf2Response<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for Pbkdf2Response<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { data })
    }
}

impl<W: Writer> Se05XCommand<W> for Pbkdf2<'_> {
    type Response<'rdata> = Pbkdf2Response<'rdata>;
}

// ************* DigestInit ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DigestInit {
    /// Serialized to TLV tag [`TAG_2`]()
    pub digest_id: CryptoObjectId,
}

impl DataSource for DigestInit {
    fn len(&self) -> usize {
        let digest_id = &Tlv::new(TAG_2, self.digest_id);
        let __data: &[&dyn DataSource] = &[digest_id];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_DEFAULT, P2_INIT, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for DigestInit {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let digest_id = &Tlv::new(TAG_2, self.digest_id);
        let __data: &[&dyn DataStream<W>] = &[digest_id];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_DEFAULT, P2_INIT, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for DigestInit {
    type Response<'rdata> = ();
}

// ************* DigestUpdate ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DigestUpdate<'data> {
    /// Serialized to TLV tag [`TAG_2`]()
    pub digest_id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
}

impl DataSource for DigestUpdate<'_> {
    fn len(&self) -> usize {
        let digest_id = &Tlv::new(TAG_2, self.digest_id);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataSource] = &[digest_id, data];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_DEFAULT, P2_UPDATE, __data, 0);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for DigestUpdate<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let digest_id = &Tlv::new(TAG_2, self.digest_id);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataStream<W>] = &[digest_id, data];
        let command = CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_DEFAULT, P2_UPDATE, __data, 0);
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for DigestUpdate<'_> {
    type Response<'rdata> = ();
}

// ************* DigestFinal ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DigestFinal<'data> {
    /// Serialized to TLV tag [`TAG_2`]()
    pub digest_id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_3`]()
    pub data: &'data [u8],
}

impl DataSource for DigestFinal<'_> {
    fn len(&self) -> usize {
        let digest_id = &Tlv::new(TAG_2, self.digest_id);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataSource] = &[digest_id, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_FINAL,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for DigestFinal<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let digest_id = &Tlv::new(TAG_2, self.digest_id);
        let data = &Tlv::new(TAG_3, self.data);
        let __data: &[&dyn DataStream<W>] = &[digest_id, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_FINAL,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DigestFinalResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub digest: &'data [u8],
}

impl<'data> Se05XResponse<'data> for DigestFinalResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (digest, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { digest })
    }
}

impl<W: Writer> Se05XCommand<W> for DigestFinal<'_> {
    type Response<'rdata> = DigestFinalResponse<'rdata>;
}

// ************* DigestOneShot ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DigestOneShot<'data> {
    /// Serialized to TLV tag [`TAG_1`]()
    pub algo: Digest,
    /// Serialized to TLV tag [`TAG_2`]()
    pub data: &'data [u8],
}

impl DataSource for DigestOneShot<'_> {
    fn len(&self) -> usize {
        let algo = &Tlv::new(TAG_1, self.algo);
        let data = &Tlv::new(TAG_2, self.data);
        let __data: &[&dyn DataSource] = &[algo, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for DigestOneShot<'_> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let algo = &Tlv::new(TAG_1, self.algo);
        let data = &Tlv::new(TAG_2, self.data);
        let __data: &[&dyn DataStream<W>] = &[algo, data];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_ONESHOT,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DigestOneShotResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub digest: &'data [u8],
}

impl<'data> Se05XResponse<'data> for DigestOneShotResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (digest, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { digest })
    }
}

impl<W: Writer> Se05XCommand<W> for DigestOneShot<'_> {
    type Response<'rdata> = DigestOneShotResponse<'rdata>;
}

// ************* GetVersion ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GetVersion {}

impl DataSource for GetVersion {
    fn len(&self) -> usize {
        let __data: &[&dyn DataSource] = &[];
        let command = CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_VERSION, __data, 11);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for GetVersion {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let __data: &[&dyn DataStream<W>] = &[];
        let command = CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_VERSION, __data, 11);
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetVersionResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub version_info: VersionInfo,
}

impl<'data> Se05XResponse<'data> for GetVersionResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (version_info, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { version_info })
    }
}

impl<W: Writer> Se05XCommand<W> for GetVersion {
    type Response<'rdata> = GetVersionResponse;
}

// ************* GetTimestamp ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GetTimestamp {}

impl DataSource for GetTimestamp {
    fn len(&self) -> usize {
        let __data: &[&dyn DataSource] = &[];
        let command = CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_TIME, __data, 20);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for GetTimestamp {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let __data: &[&dyn DataStream<W>] = &[];
        let command = CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_TIME, __data, 20);
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetTimestampResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub timestamp: &'data [u8; 12],
}

impl<'data> Se05XResponse<'data> for GetTimestampResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (timestamp, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { timestamp })
    }
}

impl<W: Writer> Se05XCommand<W> for GetTimestamp {
    type Response<'rdata> = GetTimestampResponse<'rdata>;
}

// ************* GetFreeMemory ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GetFreeMemory {
    /// Serialized to TLV tag [`TAG_1`]()
    pub memory: Memory,
}

impl DataSource for GetFreeMemory {
    fn len(&self) -> usize {
        let memory = &Tlv::new(TAG_1, self.memory);
        let __data: &[&dyn DataSource] = &[memory];
        let command = CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_MEMORY, __data, 6);
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for GetFreeMemory {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let memory = &Tlv::new(TAG_1, self.memory);
        let __data: &[&dyn DataStream<W>] = &[memory];
        let command = CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_MEMORY, __data, 6);
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetFreeMemoryResponse {
    /// Parsed from TLV tag [`TAG_1`]()
    pub available: Be<u16>,
}

impl<'data> Se05XResponse<'data> for GetFreeMemoryResponse {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (available, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { available })
    }
}

impl<W: Writer> Se05XCommand<W> for GetFreeMemory {
    type Response<'rdata> = GetFreeMemoryResponse;
}

// ************* GetRandom ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GetRandom {
    /// Serialized to TLV tag [`TAG_1`]()
    pub length: Be<u16>,
}

impl DataSource for GetRandom {
    fn len(&self) -> usize {
        let length = &Tlv::new(TAG_1, self.length);
        let __data: &[&dyn DataSource] = &[length];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_RANDOM,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for GetRandom {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let length = &Tlv::new(TAG_1, self.length);
        let __data: &[&dyn DataStream<W>] = &[length];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_RANDOM,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetRandomResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`]()
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for GetRandomResponse<'data> {
    #[inline(never)]
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = take_do_until(TAG_1, rem)?;
        let _ = rem;
        Ok(Self { data })
    }
}

impl<W: Writer> Se05XCommand<W> for GetRandom {
    type Response<'rdata> = GetRandomResponse<'rdata>;
}

// ************* DeleteAll ************* //

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DeleteAll {}

impl DataSource for DeleteAll {
    fn len(&self) -> usize {
        let __data: &[&dyn DataSource] = &[];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_DELETE_ALL,
            __data,
            ExpectedLen::Max,
        );
        command.len()
    }
    fn is_empty(&self) -> bool {
        // Command always has a header
        false
    }
}
impl<W: Writer> DataStream<W> for DeleteAll {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        let __data: &[&dyn DataStream<W>] = &[];
        let command = CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_DELETE_ALL,
            __data,
            ExpectedLen::Max,
        );
        command.to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for DeleteAll {
    type Response<'rdata> = ();
}
