use crate::Parser;

#[derive(Debug, Clone)]
pub struct OrderBookSnapshot {
    md_entry_id: i64,
    transact_time: u64,
    md_entry_px: i64,
    md_entry_size: i64,
    trade_id: i64,
    md_flags: u64,
    md_entry_type: u8,
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
