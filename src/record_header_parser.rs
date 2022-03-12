use std::fmt::{Display, Formatter};
use crate::{GlobalPcapHeader, Parser};

#[derive(Debug, Copy, Clone)]
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

    pub fn get_packet_len(&self) -> u32 {
        self.pack_length
    }
}

impl Display for RecordHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Record header: ==");
        write!(f, "Timestamp ms: {}\n", self.ts_ms);
        write!(f, "Timestamp ns: {}\n", self.ts_us);
        write!(f, "Packet length: {}\n", self.pack_length);
        writeln!(f, "Captured length: {}", self.real_length)
    }
}
