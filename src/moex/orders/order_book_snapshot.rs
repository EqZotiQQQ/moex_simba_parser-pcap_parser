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
        writeln!(f, "== OrderBookSnapshot ==");
        writeln!(f, "md_entry_id: {}", self.md_entry_id);
        writeln!(f, "transact_time: {}", self.transact_time);
        writeln!(f, "md_entry_px: {}", self.md_entry_px);
        writeln!(f, "md_entry_size: {}", self.md_entry_size);
        writeln!(f, "trade_id: {}", self.trade_id);
        writeln!(f, "md_flags: {}", self.md_flags);
        writeln!(f, "md_entry_type: {}", self.md_entry_type);
        writeln!(f, "== OrderBookSnapshot end ==")
    }
}

impl OrderBookSnapshot {
    pub const SIZE: u8 = 49;
    pub fn parse(parser: &mut Parser) -> (OrderBookSnapshot, u64) {
        (OrderBookSnapshot {
            md_entry_id: parser.next::<i64>(),
            transact_time: parser.next::<u64>(),
            md_entry_px: parser.next::<i64>(),
            md_entry_size: parser.next::<i64>(),
            trade_id: parser.next::<i64>(),
            md_flags: parser.next::<u64>(),
            md_entry_type: parser.next::<u8>(),
        }, OrderBookSnapshot::SIZE as u64)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OrderBookSnapshotPacket {
    security_id: i32,
    last_msg_seq_num_processed: u32,
    rpt_seq: u32,
    exchange_trading_session_id: u32,
    block_len: u16,
    no_md_entries: u8,
    md_entries: Vec<OrderBookSnapshot>,
}

impl OrderBookSnapshotPacket {
    pub fn parse(parser: &mut Parser) -> (OrderBookSnapshotPacket, u64) {
        let security_id = parser.next::<i32>();
        let last_msg_seq_num_processed = parser.next::<u32>();
        let rpt_seq = parser.next::<u32>();
        let exchange_trading_session_id = parser.next::<u32>();
        let block_len = parser.next::<u16>();
        let no_md_entries = parser.next::<u8>();
        let md_entries: Vec<OrderBookSnapshot> = (0..no_md_entries).map(|_| OrderBookSnapshot::parse(parser).0).collect();

        let size = block_len * no_md_entries as u16 + 19;
        (OrderBookSnapshotPacket {
            security_id,
            last_msg_seq_num_processed,
            rpt_seq,
            exchange_trading_session_id,
            block_len,
            no_md_entries,
            md_entries,
        }, size as u64)
    }
}

impl Display for OrderBookSnapshotPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Security ID: {}", self.security_id);
        writeln!(f, "Sequential number of last message: {}", self.last_msg_seq_num_processed);
        writeln!(f, "Rqt sequential: {}", self.rpt_seq);
        writeln!(f, "Exchange trading session ID: {}", self.exchange_trading_session_id);
        writeln!(f, "Block length: {}", self.block_len);
        writeln!(f, "Number of entries: {}", self.no_md_entries);
        for (i, entry) in self.md_entries.iter().enumerate() {
            writeln!(f, "Entry number {}:\n{}", i, entry);
        }
        writeln!(f)
    }
}
