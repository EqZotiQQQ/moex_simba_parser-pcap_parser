use std::fmt::{Display, Formatter};
use crate::errors::CustomErrors;
use crate::parser::{Parser};

#[allow(unused_must_use)]
#[derive(Debug)]
pub enum Ordering {
    BigEndianNanoseconds(u32),
    BigEndianMilliseconds(u32),
    LittleEndianNanoseconds(u32),
    LittleEndMilliseconds(u32),
}

impl Ordering {
    pub fn new(magic: u32) -> Result<Ordering, CustomErrors> {
        Ok(match magic {
            0xA1B2C3D4 => Ordering::BigEndianNanoseconds(0xA1B2C3D4),
            0xA1B23C4D => Ordering::BigEndianMilliseconds(0xA1B23C4D),
            0xD4C3B2A1 => Ordering::LittleEndianNanoseconds(0xD4C3B2A1),
            0x4D3CB2A1 => Ordering::LittleEndMilliseconds(0x4D3CB2A1),
            _ => return Err(CustomErrors::BadMagicNumberError),
        })
    }
}

impl Display for Ordering {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Ordering::BigEndianNanoseconds(v) => write!(f, "Big endian - Nanoseconds ({:#02X})", v),
            Ordering::BigEndianMilliseconds(v) => write!(f, "Big endian - Milliseconds ({:#02X})", v),
            Ordering::LittleEndianNanoseconds(v) => write!(f, "Little endian - Nanoseconds ({:#02X})", v),
            Ordering::LittleEndMilliseconds(v) => write!(f, "Little endian - Milliseconds ({:#02X})", v),
        }
    }
}

#[derive(Debug)]
pub struct GlobalPcapHeader {
     magic_number: Ordering,
     version_major: u16,
     version_minor: u16,
     time_zone: i32,
     sig_figs: u32,
     snap_len: u32,
     network: u32,
}

impl GlobalPcapHeader {
    pub fn parse(parser: &mut Parser) -> Result<GlobalPcapHeader, CustomErrors> {
        let magic_number = Ordering::new(parser.next_be::<u32>()?)?;
        parser.set_endian(&magic_number);

        Ok(GlobalPcapHeader {
            magic_number,
            version_major: parser.next::<u16>()?,
            version_minor: parser.next::<u16>()?,
            time_zone: parser.next::<i32>()?,
            sig_figs: parser.next::<u32>()?,
            snap_len: parser.next::<u32>()?,
            network: parser.next::<u32>()?,
        })
    }
}

#[allow(unused_must_use)]
impl Display for GlobalPcapHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Global pcap header: ==");
        write!(f, "Magic number: {}\n", self.magic_number);
        write!(f, "Major version: {}\n", self.version_major);
        write!(f, "Minor version: {}\n", self.version_minor);
        write!(f, "Time zone: {}\n", self.time_zone);
        write!(f, "Sig figs: {}\n", self.sig_figs);
        write!(f, "Snap len: {}\n", self.snap_len);
        writeln!(f, "Network: {}", self.network);
        writeln!(f, "== GlobalPcapHeader end ==")
    }
}
