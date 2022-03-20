use std::fmt::{Display, Formatter};
use crate::{CustomErrors, Parser};
use crate::utils::utils::from_ms_ns;

#[derive(Debug, Copy, Clone)]
pub struct RecordHeader {
    ts_ms: u32,
    ts_us: u32,
    pack_length: u32,
    real_length: u32,
}

impl RecordHeader {
    pub fn parse(parser: &mut Parser) -> Result<RecordHeader, CustomErrors> {
        let ts_ms = parser.next::<u32>()?;
        let ts_us = parser.next::<u32>()?;
        let pack_length = parser.next::<u32>()?;
        let real_length = parser.next::<u32>()?;
        Ok(RecordHeader {
            ts_ms,
            ts_us,
            pack_length,
            real_length,
        })
    }

    pub fn get_packet_len(&self) -> u32 {
        self.pack_length
    }
}

#[allow(unused_must_use)]
impl Display for RecordHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Record header ==");
        writeln!(f, "Timestamp ms: {}", self.ts_ms);
        writeln!(f, "Timestamp ns: {}", self.ts_us);
        writeln!(f, "Transaction date: {}", from_ms_ns(self.ts_ms as u64, self.ts_us as u64));
        writeln!(f, "Packet length: {} bytes", self.pack_length);
        writeln!(f, "Captured length: {} bytes", self.real_length);
        writeln!(f, "== Record header end ==")
    }
}
