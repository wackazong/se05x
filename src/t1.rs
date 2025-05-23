// Copyright (C) 2023 Nitrokey GmbH
// SPDX-License-Identifier: LGPL-3.0-only

use hex_literal::hex;
use iso7816::command::writer::IntoWriter;
use iso7816::command::Writer;

pub type Crc = crc16::State<crc16::X_25>;

use core::fmt::{self, Debug};
use core::ops::Not;

use crate::embedded_hal::{
    i2c::{Read, Write, WriteRead},
    Delay,
};
use crate::macros::enum_u8;

mod i2cimpl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Atr<'a> {
    /// Protocol version only `01` is supported
    pub pver: u8,
    /// VendorID,
    pub vid: &'a [u8; 5],
    /// Block waiting time
    pub bwt: u16,
    /// Maximum Information Field Size of the SE
    pub ifsc: u16,
    /// Maximum I2C clock frequency (kHz)
    pub plid: u8,
    /// Maximal I2C Clock Frequency
    pub mcf: u16,
    pub config: u8,
    /// Minimum polling time (ms)
    pub mpot: u8,
    /// Secure element guard time (microseconds)
    pub segt: u16,
    /// Wake-up time (microseconds)
    pub wut: u16,
    pub historical_bytes: &'a [u8],
}

impl Default for Atr<'_> {
    fn default() -> Self {
        Self {
            pver: 1,
            vid: &hex!("FFFFFFFFFF"),
            bwt: 0,
            ifsc: MAX_FRAME_DATA_LEN as _,
            plid: 0,
            mcf: 0,
            config: 0,
            mpot: 1,
            segt: SEGT_US as _,
            wut: 0,
            historical_bytes: &[],
        }
    }
}

impl<'a> Atr<'a> {
    /// If fails to parse, returns default values
    pub fn parse(data: &'a [u8]) -> Result<Self, Error> {
        // let atr = hex!("00a0000003960403e800fe020b03e80801000000006400000a4a434f5034204154504f");
        debug!("Parsing atr: {data:02x?}");
        if data.len() < 7 {
            error!("ATR Error 1");
            return Err(Error::Line(line!()));
        }
        let pver = data[0];
        let vid: &[u8; 5] = (&data[1..][..5]).try_into().unwrap();
        let dllp_len = data[6];

        let rem = &data[7..];

        if rem.len() < dllp_len as usize || dllp_len < 2 {
            error!("ATR Error 2");
            return Err(Error::Line(line!()));
        }
        let (dllp, rem) = rem.split_at(dllp_len as usize);

        let [bwt1, bwt2, ifsc1, ifsc2, ..] = dllp else {
            error!("ATR Error 3");
            return Err(Error::Line(line!()));
        };
        let bwt = u16::from_be_bytes([*bwt1, *bwt2]);
        let ifsc = u16::from_be_bytes([*ifsc1, *ifsc2]);

        if rem.len() < 2 {
            error!("ATR Error 4");
            return Err(Error::Line(line!()));
        }

        let plid = rem[0];
        let plp_len = rem[1];
        let rem = &rem[2..];
        if rem.len() < plp_len as usize {
            error!("ATR Error 6");
            return Err(Error::Line(line!()));
        }
        let (plp, rem) = rem.split_at(plp_len as usize);
        let [mcf1, mcf2, config, mpot, _rfu1, _rfu2, _rfu3, segt1, segt2, wut1, wut2, ..] = plp
        else {
            error!("ATR Error 7");
            return Err(Error::Line(line!()));
        };
        let mcf = u16::from_be_bytes([*mcf1, *mcf2]);
        let segt = u16::from_be_bytes([*segt1, *segt2]);
        let wut = u16::from_be_bytes([*wut1, *wut2]);

        if rem.is_empty() {
            error!("ATR Error 8");
            return Err(Error::Line(line!()));
        }
        let hb_len = rem[0];
        let rem = &rem[1..];
        if rem.len() < hb_len as usize {
            error!("ATR Error 9");
            return Err(Error::Line(line!()));
        }

        let historical_bytes = &rem[..hb_len as usize];

        Ok(Self {
            pver,
            vid,
            bwt,
            ifsc,
            plid,
            mcf,
            config: *config,
            mpot: *mpot,
            segt,
            wut,
            historical_bytes,
        })
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Seq(bool);

impl Seq {
    pub const ZERO: Self = Seq(false);
    pub const ONE: Self = Seq(true);
}

impl Not for Seq {
    type Output = Self;
    fn not(self) -> Self::Output {
        Seq(!self.0)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Pcb {
    I(Seq, bool),        // seq, multi
    S(SBlock),           // code, response?
    R(Seq, RBlockError), // seq, err
}

enum_u8!(
    #[rustfmt::skip]
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum SBlock {
        ResyncRequest =              0b11000000,
        ResyncResponse =             0b11100000,
        IfsRequest =                 0b11000001,
        IfsResponse =                0b11100001,
        AbortRequest =               0b11000010,
        AbortResponse =              0b11100010,
        WtxRequest =                 0b11000011,
        WtxResponse =                0b11100011,
        InterfaceSoftResetRequest =  0b11001111,
        InterfaceSoftResetResponse = 0b11101111,
        EndOfApduSessionRequest =    0b11000101,
        EndOfApduSessionResponse =   0b11100101,
        SeChipResetRequest =         0b11000110,
        SeChipResetResponse =        0b11100110,
        GetAtrRequest =              0b11000111,
        GetAtrResponse =             0b11100111,
    }
);

enum_u8!(
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum RBlockError {
        #![mask(0b11)]
        NoError = 0b00,
        CrcError = 0b01,
        OtherError = 0b10,
    }
);

/// PCB Mask
const I_BLOCK_PCB_MASK: u8 = 0b10011111;
/// PCB template
const I_BLOCK_PCB: u8 = 0b00000000;
/// SEQ mask
const I_BLOCK_SEQ: u8 = 0b01000000;
/// MORE mask
const I_BLOCK_MOR: u8 = 0b00100000;

/// PCB template
const R_BLOCK_PCB: u8 = 0b10000000;
/// PCB template
const R_BLOCK_PCB_MASK: u8 = 0b11101100;
/// SEQ mask
const R_BLOCK_SEQ: u8 = 0b00010000;
/// CRC mask
const R_BLOCK_ERROR_MASK: u8 = 0b00000011;

impl Pcb {
    pub fn to_byte(self) -> u8 {
        match self {
            Self::I(seq, multi) => Self::i_pcb(seq, multi),
            Self::S(block) => Self::s_pcb(block),
            Self::R(seq, err) => Self::r_pcb(seq, err),
        }
    }

    pub fn i_pcb(seq: Seq, multi: bool) -> u8 {
        let mut pcb = I_BLOCK_PCB;
        if multi {
            pcb |= I_BLOCK_MOR;
        }

        if seq == Seq::ONE {
            pcb |= I_BLOCK_SEQ;
        }
        pcb
    }
    pub fn s_pcb(block: SBlock) -> u8 {
        block.into()
    }
    pub fn r_pcb(seq: Seq, error: RBlockError) -> u8 {
        let mut pcb = R_BLOCK_PCB;
        if seq == Seq::ONE {
            pcb |= R_BLOCK_SEQ;
        }

        pcb |= error as u8;
        pcb
    }

    pub fn parse(value: u8) -> Result<Self, Error> {
        if value & I_BLOCK_PCB_MASK == I_BLOCK_PCB {
            let seq = if value & I_BLOCK_SEQ == 0 {
                Seq::ZERO
            } else {
                Seq::ONE
            };

            let more = (value & I_BLOCK_MOR) != 0;
            return Ok(Self::I(seq, more));
        }

        if value & R_BLOCK_PCB_MASK == R_BLOCK_PCB {
            let seq = if value & R_BLOCK_SEQ == 0 {
                Seq::ZERO
            } else {
                Seq::ONE
            };
            let error = (value & R_BLOCK_ERROR_MASK)
                .try_into()
                .map_err(|_| Error::BadPcb)?;
            return Ok(Self::R(seq, error));
        }

        if let Ok(sblock) = value.try_into() {
            return Ok(Self::S(sblock));
        }

        Err(Error::BadPcb)
    }
}

pub trait I2CErrorNack: Debug {
    fn is_address_nack(&self) -> bool;
    fn is_data_nack(&self) -> bool;
}
pub trait I2CForT1:
    Read<u8, Error = <Self as I2CForT1>::Error>
    + Write<u8, Error = <Self as I2CForT1>::Error>
    + WriteRead<u8, Error = <Self as I2CForT1>::Error>
{
    type Error: I2CErrorNack;
}

impl<T> I2CForT1 for T
where
    T: Read<u8>
        + Write<u8, Error = <T as Read<u8>>::Error>
        + WriteRead<u8, Error = <T as Read<u8>>::Error>,
    <T as Read<u8>>::Error: I2CErrorNack,
{
    type Error = <T as Read<u8>>::Error;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unknown,
    AddressNack,
    DataNack,
    BadCrc,
    BadPcb,
    BadAddress,
    ReceptionBuffer,
    Line(u32),
    Timeout,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => f.write_str("Unknown T=1 error"),
            Self::AddressNack => f.write_str("NACK on from the device address"),
            Self::DataNack => f.write_str("Nack for data write"),
            Self::BadCrc => f.write_str("CRC error"),
            Self::BadPcb => f.write_str("Received invalid PCB"),
            Self::BadAddress => f.write_str("Bad address"),
            Self::ReceptionBuffer => f.write_str("Reception buffer is too small"),
            Self::Timeout => f.write_str("Read timed out"),
            Self::Line(l) => write!(f, "Error comming from line: {l}"),
        }
    }
}

impl iso7816::command::writer::Error for Error {
    fn failed_serialization(_cause: &'static str) -> Self {
        error!("Failed serializaiton: {}", _cause);
        Self::Line(line!())
    }
}

pub struct T1oI2C<Twi, D> {
    twi: Twi,
    se_address: u8,
    nad_hd2se: u8,
    nad_se2hd: u8,
    iseq_snd: Seq,
    iseq_rcv: Seq,
    /// Waiting time between attempts to read
    ///
    /// Microseconds
    mpot: u32,
    /// Retry count for attempts to write data to the se
    pub retry_count: u32,
    delay: D,
    segt: u32,
    /// Block waiting time
    /// Maximum time the se05x can take to respond
    ///
    /// Microseconds
    bwt: u32,
}

// const TWI_RETRIES: usize = 128;
// const TWI_RETRY_DELAY_MS: u32 = 2;
// const TWI_RETRY_DELAY_US: u32 = TWI_RETRY_DELAY_MS * 1000;
/// SEGT value in microseconds
/// Minimun time between reading attempts
const SEGT_US: u32 = 10;
const BWT_US: u32 = 100_000;

/// See table 4 of UM1225
const NAD_HD_TO_SE: u8 = 0x5A;
/// See table 4 of UM1225
const NAD_SE_TO_HD: u8 = 0xA5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataReceived {
    /// Received one or more IBlocks
    ///
    /// Returns the size of data written to the reception buffer
    IBlocks(usize),
    SBlock {
        block: SBlock,
        /// Any data written prior to receiving the s block
        i_data: usize,
        s_data: usize,
    },
}

const DEFAULT_RETRY_COUNT: u32 = 1024;

#[cfg(feature = "embedded-hal-v0.2.7")]
impl<M, N, E> T1oI2C<crate::embedded_hal::Hal027<M>, crate::embedded_hal::Hal027<N>>
where
    N: embedded_hal_v0_2_7::blocking::delay::DelayUs<u32>,
    M: embedded_hal_v0_2_7::blocking::i2c::Write<Error = E>
        + embedded_hal_v0_2_7::blocking::i2c::Read<Error = E>
        + embedded_hal_v0_2_7::blocking::i2c::WriteRead<Error = E>,
    E: I2CErrorNack,
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
impl<M, N, E> T1oI2C<crate::embedded_hal::Hal10<M>, crate::embedded_hal::Hal10<N>>
where
    N: embedded_hal_v1_0::delay::DelayNs,
    M: embedded_hal_v1_0::i2c::I2c<Error = E>,
    E: I2CErrorNack,
{
    pub fn new_hal_10(twi: M, se_address: u8, delay: N) -> Self {
        Self::new(
            crate::embedded_hal::Hal10(twi),
            se_address,
            crate::embedded_hal::Hal10(delay),
        )
    }
}

impl<Twi: I2CForT1, D: Delay> T1oI2C<Twi, D> {
    pub fn new(twi: Twi, se_address: u8, delay: D) -> Self {
        // Default MPOT value.
        // TODO: get from ATR
        const DMPOT_MS: u32 = 1;
        Self {
            twi,
            se_address,
            // See table 4
            nad_hd2se: NAD_HD_TO_SE,
            nad_se2hd: NAD_SE_TO_HD,
            iseq_snd: Seq::ZERO,
            iseq_rcv: Seq::ZERO,
            mpot: DMPOT_MS * 1000,
            segt: SEGT_US as _,
            retry_count: DEFAULT_RETRY_COUNT,
            bwt: BWT_US,
            delay,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        trace!("Writing");
        match self.twi.write(self.se_address, data) {
            Ok(_) => Ok(()),
            Err(err) if err.is_address_nack() => Err(Error::AddressNack),
            Err(err) if err.is_data_nack() => Err(Error::DataNack),
            Err(_err) => {
                warn!("Got error: {:?}", _err);
                Err(Error::Line(line!()))
            }
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        match self.twi.read(self.se_address, buffer) {
            Ok(_) => Ok(()),
            Err(err) if err.is_address_nack() => Err(Error::AddressNack),
            Err(err) if err.is_data_nack() => Err(Error::DataNack),
            Err(_err) => {
                warn!("Got error: {:?}", _err);
                Err(Error::Line(line!()))
            }
        }
    }

    // Not actually used as discouraged by 3.1.1.1
    pub fn write_read(&mut self, data: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        match self.twi.write_read(self.se_address, data, buffer) {
            Ok(_) => Ok(()),
            Err(err) if err.is_address_nack() => Err(Error::AddressNack),
            Err(err) if err.is_data_nack() => Err(Error::DataNack),
            Err(_err) => {
                warn!("Unknown error when writing & reading: {:?}", _err);
                Err(Error::Line(line!()))
            }
        }
    }

    pub fn receive_data(&mut self, buffer: &mut [u8]) -> Result<DataReceived, Error> {
        let mut written = 0;
        let mut retry_count = self.bwt / self.mpot + 1;
        let mut i = 0;
        loop {
            let mut header_buffer = [0; HEADER_LEN];
            let mut crc_buf = [0; TRAILER_LEN];
            i += 1;
            if i == retry_count {
                break;
            }

            let read = self.read(&mut header_buffer);
            match read {
                Ok(()) => {}
                Err(Error::AddressNack) => {
                    self.wait_mpot();
                    continue;
                }
                Err(err) => {
                    return Err(err);
                }
            }

            let [nad, pcb, len] = header_buffer;
            debug!("Received header: {:02x?}", header_buffer);

            if buffer.len() < written + len as usize {
                error!("Buffer too small");
                return Err(Error::ReceptionBuffer);
            }

            if len as usize > MAX_FRAME_DATA_LEN {
                error!("Frame too large");
                return Err(Error::ReceptionBuffer);
            }

            let mut data_buf = [0; MAX_FRAME_DATA_LEN];
            let current_buf = &mut buffer[written..][..len as usize];
            let data_buf = &mut data_buf[..len as _];

            if nad != self.nad_se2hd {
                error!("Received bad nad: {:02x}", nad);
                return Err(Error::BadAddress);
            }

            if len != 0 {
                self.read(data_buf)?;
            }
            self.read(&mut crc_buf)?;

            let pcb = Pcb::parse(pcb).map_err(|_| Error::BadPcb)?;

            let mut crc = Crc::new();
            crc.update(&header_buffer);
            crc.update(data_buf);
            let crc = crc.get().to_le_bytes();
            if crc_buf != crc {
                error!("Got bad crc: {:02x?} expected {:02x?}", &data_buf[..2], crc);
                // TODO: write R-Block with error
                return Err(Error::BadCrc);
            }

            let (seq, more) = match pcb {
                Pcb::S(SBlock::WtxRequest) => {
                    if len != 1 {
                        return Err(Error::Line(line!()));
                    }
                    let mult = data_buf[0];
                    debug!("Got WtxRequest, {mult}");
                    let frame = [
                        self.nad_hd2se,
                        Pcb::S(SBlock::WtxResponse).to_byte(),
                        1,
                        mult,
                    ];
                    let [crc1, crc2] = Crc::calculate(&frame).to_le_bytes();
                    self.write(&[frame[0], frame[1], frame[2], frame[3], crc1, crc2])?;

                    retry_count = (self.bwt * mult as u32) / self.mpot + 1;
                    i = 0;
                    self.delay.delay_us(100_000);
                    continue;
                }
                Pcb::S(block) => {
                    current_buf.copy_from_slice(data_buf);
                    return Ok(DataReceived::SBlock {
                        block,
                        i_data: written,
                        s_data: len as usize,
                    });
                }
                Pcb::R(_, _) => {
                    error!("Got unexpected R-Block in receive");
                    return Err(Error::Line(line!()));
                }
                Pcb::I(seq, more) => (seq, more),
            };
            current_buf.copy_from_slice(data_buf);
            written += len as usize;

            if seq != self.iseq_rcv {
                warn!("Got bad seq");
            }
            self.iseq_rcv = !seq;

            if !more {
                return Ok(DataReceived::IBlocks(written));
            }
            let frame = [
                self.nad_hd2se,
                Pcb::R(!seq, RBlockError::NoError).to_byte(),
                0,
            ];
            let [crc1, crc2] = Crc::calculate(&frame).to_le_bytes();
            self.write(&[frame[0], frame[1], frame[2], crc1, crc2])?;
        }
        error!("Waited for btw");
        Err(Error::Timeout)
    }

    pub fn resync(&mut self) -> Result<(), Error> {
        trace!("Resync");
        let header = [self.nad_hd2se, Pcb::S(SBlock::ResyncRequest).to_byte(), 0];
        let [crc1, crc2] = Crc::calculate(&header).to_le_bytes();
        let frame = [header[0], header[1], header[2], crc1, crc2];
        debug!("Sending: {frame:02x?}");
        self.write(&frame)?;
        self.wait_segt();
        let data = self.receive_data(&mut [])?;
        if !matches!(
            data,
            DataReceived::SBlock {
                block: SBlock::ResyncResponse,
                i_data: 0,
                s_data: 0
            }
        ) {
            error!("Got unexpected error: {data:?}");
            return Err(Error::BadPcb);
        }
        self.iseq_snd = Seq::ZERO;
        self.iseq_rcv = Seq::ZERO;
        Ok(())
    }

    // TODO: find proper length for buffer
    pub fn interface_soft_reset<'buf>(
        &mut self,
        buffer: &'buf mut [u8; 64],
    ) -> Result<Atr<'buf>, Error> {
        trace!("Interface Soft Reset");
        let header = [
            self.nad_hd2se,
            Pcb::S(SBlock::InterfaceSoftResetRequest).to_byte(),
            0,
        ];
        let [crc1, crc2] = Crc::calculate(&header).to_le_bytes();
        self.write(&[header[0], header[1], header[2], crc1, crc2])?;
        self.wait_segt();
        let data = self.receive_data(buffer)?;
        let received = if let DataReceived::SBlock {
            block: SBlock::InterfaceSoftResetResponse,
            i_data: 0,
            s_data,
        } = data
        {
            s_data
        } else {
            error!("Got unexpected error: {data:?}");
            return Err(Error::BadPcb);
        };
        let atr = Atr::parse(&buffer[..received]);
        if let Ok(atr) = &atr {
            let mpot: u32 = atr.mpot.into();
            self.mpot = 1000 * mpot;
            self.segt = atr.segt.into();
            self.bwt = (atr.bwt as u32) * 1000;
        };
        self.iseq_snd = Seq::ZERO;
        self.iseq_rcv = Seq::ZERO;
        debug_now!("Got atr: {atr:?}");
        Ok(atr.unwrap_or_default())
    }

    pub fn wait_segt(&mut self) {
        self.delay.delay_us(self.segt)
    }

    pub fn wait_mpot(&mut self) {
        self.delay.delay_us(self.mpot)
    }
}

/// UM1225 2.1.1
const MAX_FRAME_DATA_LEN: usize = 0xFE;
const HEADER_LEN: usize = 3;
const TRAILER_LEN: usize = 2;
const MAX_FRAME_LEN: usize = MAX_FRAME_DATA_LEN + HEADER_LEN + TRAILER_LEN;

pub struct FrameSender<'writer, Twi, D> {
    writer: &'writer mut T1oI2C<Twi, D>,
    /// Total amount of application data that will be written
    data: usize,
    /// Amount of application data already written, includes data currently in `current_frame_buffer`
    written: usize,
    sent: usize,
    current_frame_buffer: [u8; MAX_FRAME_LEN],
}

impl<'writer, Twi: I2CForT1, D: Delay> FrameSender<'writer, Twi, D> {
    fn current_offset(&self) -> usize {
        debug_assert!(self.written - self.sent <= MAX_FRAME_LEN);
        self.written - self.sent
    }

    pub fn new(writer: &'writer mut T1oI2C<Twi, D>, data: usize) -> Self {
        Self {
            writer,
            data,
            written: 0,
            sent: 0,
            current_frame_buffer: [0; MAX_FRAME_LEN],
        }
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<usize, Error> {
        // Prevent false positive when delog is disabled
        #[allow(clippy::if_same_then_else)]
        if data.len() < 10 {
            debug!("Writing data: {:02x?}", data);
        } else {
            debug!("Writing {} bytes", data.len());
        }

        if data.is_empty() {
            return Ok(0);
        }
        if data.len() + self.written > self.data {
            error!("Writing more data than expected");
            return Err(Error::Line(line!()));
        }

        let current_offset = self.current_offset();
        let available_in_frame = MAX_FRAME_DATA_LEN - current_offset;
        let chunk_len = available_in_frame.min(data.len());
        let chunk = &data[..chunk_len];
        self.written += chunk_len;
        self.current_frame_buffer[HEADER_LEN + current_offset..][..chunk_len]
            .copy_from_slice(chunk);

        // frame is full, must flush
        let full_frame = chunk_len == available_in_frame;
        // fully written, send remaining buffered data
        let final_data = self.written == self.data;

        if full_frame || final_data {
            self.send_current_frame()?;
        }

        Ok(chunk_len)
    }

    pub fn send_current_frame(&mut self) -> Result<(), Error> {
        let data_len = self.current_offset();
        let is_last = self.written == self.data;
        let pcb = Pcb::I(self.writer.iseq_snd, !is_last).to_byte();

        self.writer.iseq_snd = !self.writer.iseq_snd;

        let header = [self.writer.nad_hd2se, pcb, data_len as u8];
        self.current_frame_buffer[0..HEADER_LEN].copy_from_slice(&header);
        let trailer =
            Crc::calculate(&self.current_frame_buffer[..HEADER_LEN + data_len]).to_le_bytes();
        self.current_frame_buffer[HEADER_LEN + data_len..][..TRAILER_LEN].copy_from_slice(&trailer);
        trace!(
            "Sending:\n\tHeader: {:02x?}\n\tData: {:02x?}\n\tTrailer: {:02x?}",
            &self.current_frame_buffer[..HEADER_LEN],
            &self.current_frame_buffer[HEADER_LEN..][..data_len],
            &self.current_frame_buffer[HEADER_LEN + data_len..][..TRAILER_LEN],
        );

        let mut wrote_success = false;
        for _ in 0..self.writer.retry_count {
            match self
                .writer
                .write(&self.current_frame_buffer[..data_len + HEADER_LEN + TRAILER_LEN])
            {
                Ok(()) => {
                    wrote_success = true;
                    break;
                }
                // Err(Error::DataNack) => {
                //     self.writer.wait_segt();
                //     continue;
                // }
                Err(Error::AddressNack) => {
                    self.writer.wait_segt();
                    continue;
                }
                Err(e) => return Err(e),
            }
        }

        if !wrote_success {
            debug_now!(
                "Failed to send data after {} tries",
                self.writer.retry_count
            );
            return Err(Error::Timeout);
        }

        self.sent += data_len;

        if is_last {
            // No R-BLOCK expected for non chained I block
            return Ok(());
        }

        let mut resp_buf = [0u8; 5];
        self.writer.wait_segt();
        self.writer.read(&mut resp_buf)?;
        debug!("Got R-Block: {:02x?}", resp_buf);
        let [nad, pcb, len, crc1, crc2] = resp_buf;

        if nad != self.writer.nad_se2hd {
            error!("Received bad nad: {:02x}", nad);
            return Err(Error::BadAddress);
        }

        let pcb = Pcb::parse(pcb);

        match pcb {
            Ok(Pcb::R(seq, RBlockError::NoError)) if seq == self.writer.iseq_snd => {}
            Ok(Pcb::R(_, RBlockError::NoError)) => {
                warn!("Got incorrect expected sequence");
            }
            Ok(Pcb::R(_, RBlockError::CrcError)) => {
                error!("Got CrcError");
                return Err(Error::BadCrc);
            }
            _ => {
                error!("Got bad PCB: {pcb:?}");
                return Err(Error::BadPcb);
            }
        }

        if len != 0 {
            error!("Received R-block with bad len: {}", len);
            return Err(Error::BadAddress);
        }

        let crc = Crc::calculate(&resp_buf[0..HEADER_LEN]).to_le_bytes();
        if [crc1, crc2] != crc {
            error!(
                "Got bad crc. Got {:02x?}, expected {:02x?}",
                [crc1, crc2],
                crc
            );
            return Err(Error::BadCrc);
        }

        Ok(())
    }
}

impl<Twi: I2CForT1, D: Delay> Writer for FrameSender<'_, Twi, D> {
    type Error = Error;
    fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error> {
        self.write_data(data)
    }
}

impl<'writer, Twi: I2CForT1, D: Delay> IntoWriter for &'writer mut T1oI2C<Twi, D> {
    type Writer = FrameSender<'writer, Twi, D>;
    fn into_writer(self, to_write: usize) -> Result<Self::Writer, <Self::Writer as Writer>::Error> {
        Ok(FrameSender::new(self, to_write))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_round_trip(value: u8, pcb: Pcb) {
        assert_eq!(
            value,
            pcb.to_byte(),
            "Expected 0b{value:08b}, got 0b{:08b}",
            pcb.to_byte()
        );
        assert_eq!(pcb, Pcb::parse(value).unwrap());
    }

    #[test]
    fn i_pcb() {
        assert_round_trip(0b01100000, Pcb::I(Seq::ONE, true));
        assert_round_trip(0b01000000, Pcb::I(Seq::ONE, false));
        assert_round_trip(0b00100000, Pcb::I(Seq::ZERO, true));
        assert_round_trip(0b00000000, Pcb::I(Seq::ZERO, false));
    }

    #[test]
    fn r_pcb() {
        assert_round_trip(0b10010000, Pcb::R(Seq::ONE, RBlockError::NoError));
        assert_round_trip(0b10000000, Pcb::R(Seq::ZERO, RBlockError::NoError));

        assert_round_trip(0b10010001, Pcb::R(Seq::ONE, RBlockError::CrcError));
        assert_round_trip(0b10000001, Pcb::R(Seq::ZERO, RBlockError::CrcError));

        assert_round_trip(0b10010010, Pcb::R(Seq::ONE, RBlockError::OtherError));
        assert_round_trip(0b10000010, Pcb::R(Seq::ZERO, RBlockError::OtherError));
    }

    #[test]
    fn atr() {
        let atr: [u8; 0x23] = hex!(
                "00" // protocol version
                "a000000396" // vendor id
                "04" // DLLP length
                    "03e8" // BWT = 03E8 = 1s
                    "00fe" // IFSC = 00FE = default
                "02" //PLID
                "0b"// PLP length
                    "03e8" // Max frequency: 1MHz
                    "08" // Config: HS mode supported
                    "01" // MPOT  = 1 ms
                    "000000" // RFU
                    "0064" // SEGT = 100ms
                    "0000" // WUT = 0ms
                "0a" // len of historical bytes
                    "4a434f5034204154504f"
        );
        assert_eq!(
            Atr::parse(&atr).unwrap(),
            Atr {
                pver: 0,
                vid: &hex!("a000000396"),
                bwt: 1000,
                ifsc: 0xFE,
                plid: 2,
                mcf: 1000,
                config: 0x08,
                mpot: 1,
                segt: 100,
                wut: 0,
                historical_bytes: &hex!("4a434f5034204154504f")
            }
        );
    }
}
