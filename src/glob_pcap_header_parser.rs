use std::fmt::{Display, Formatter};
use crate::errors::CustomErrors;
use crate::parser::{Endian, Parser};

const BIG_ENDIAN_MILLISECONDS: u32 = 0xA1B2C3D4;
const BIG_ENDIAN_NANOSECONDS: u32 = 0xA1B23C4D;
const LITTLE_ENDIAN_MILLISECONDS: u32 = 0xD4C3B2A1;
const LITTLE_ENDIAN_NANOSECONDS: u32 = 0x4D3CB2A1;

#[derive(Debug)]
pub struct GlobalPcapHeader {
     magic_number: u32,
     version_major: u16,
     version_minor: u16,
     time_zone: i32,
     sig_figs: u32,
     snap_len: u32,
     network: u32,
}

impl GlobalPcapHeader {
    pub fn parse(parser: &mut Parser) -> Result<GlobalPcapHeader, CustomErrors> {
        let magic_number = parser.next_be::<u32>();
        let endian = match magic_number {
            BIG_ENDIAN_MILLISECONDS | BIG_ENDIAN_NANOSECONDS => Endian::Big,
            LITTLE_ENDIAN_MILLISECONDS | LITTLE_ENDIAN_NANOSECONDS => Endian::Little,
            _ => return Err(CustomErrors::BadMagicNumberError)
        };
        parser.set_endian(endian);

        Ok(GlobalPcapHeader {
            magic_number,
            version_major: parser.next::<u16>(),
            version_minor: parser.next::<u16>(),
            time_zone: parser.next::<i32>(),
            sig_figs: parser.next::<u32>(),
            snap_len: parser.next::<u32>(),
            network: parser.next::<u32>(),
        })
    }
}

impl Display for GlobalPcapHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Global pcap header: ==");
        write!(f, "Magic number: {}\n", self.magic_number);
        write!(f, "Major version: {}\n", self.version_major);
        write!(f, "Minor version: {}\n", self.version_minor);
        write!(f, "Time zone: {}\n", self.time_zone);
        write!(f, "Sig figs: {}\n", self.sig_figs);
        writeln!(f, "Network: {}", self.network)
    }
}
