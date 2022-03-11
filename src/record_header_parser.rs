use std::fmt::{Display, Formatter};
use crate::{GlobalPcapHeader, Parser};

pub struct RecordHeader {
    ts_ms: u32,
    ts_us: u32,
    pack_length: u32,
    real_length: u32,
}

impl RecordHeader {
    pub fn parse(parser: &mut Parser) -> RecordHeader {
        RecordHeader {
            ts_ms: parser.next::<u32>(),
            ts_us: parser.next::<u32>(),
            pack_length: parser.next::<u32>(),
            real_length: parser.next::<u32>(),
        }
    }
}

impl Display for RecordHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timestamp ms: {}\n", self.ts_ms);
        write!(f, "Timestamp ns: {}\n", self.ts_us);
        write!(f, "Packet length: {}\n", self.pack_length);
        writeln!(f, "Captured length: {}", self.real_length)
    }
}
