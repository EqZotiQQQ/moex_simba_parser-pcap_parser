use std::fmt::{Display, Formatter};
use crate::Parser;

#[derive(Debug, Clone, Copy)]
pub struct OrderBookSnapshot {
    md_entry_id: i64,
    transact_time: u64,
    md_entry_px: i64,
    md_entry_size: i64,
    trade_id: i64,
    md_flags: u64,
    md_entry_type: u8,
}

#[allow(unused_must_use)]
impl Display for OrderBookSnapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "md_entry_id: {}", self.md_entry_id);
        writeln!(f, "transact_time: {}", self.transact_time);
        writeln!(f, "md_entry_px: {}", self.md_entry_px);
        writeln!(f, "md_entry_size: {}", self.md_entry_size);
        writeln!(f, "trade_id: {}", self.trade_id);
        writeln!(f, "md_flags: {}", self.md_flags);
        writeln!(f, "md_entry_type: {}", self.md_entry_type)
    }
}

impl OrderBookSnapshot {
    pub const SIZE: u8 = 49;
    pub fn parse(parser: &mut Parser) -> OrderBookSnapshot {
        OrderBookSnapshot {
            md_entry_id: parser.next::<i64>(),
            transact_time: parser.next::<u64>(),
            md_entry_px: parser.next::<i64>(),
            md_entry_size: parser.next::<i64>(),
            trade_id: parser.next::<i64>(),
            md_flags: parser.next::<u64>(),
            md_entry_type: parser.next::<u8>(),
        }
    }
}
