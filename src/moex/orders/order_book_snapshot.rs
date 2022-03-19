use std::fmt::{Display, Formatter};
use crate::moex::orders::details::details::MDEntryType;
use crate::Parser;

#[derive(Debug, Clone, Copy)]
pub struct OrderBookSnapshot {
    md_entry_id: i64,
    transact_time: u64,
    md_entry_px: i64,
    md_entry_size: i64,
    trade_id: i64,
    md_flags: u64,
    md_entry_type: MDEntryType,
}

#[allow(unused_must_use)]
impl Display for OrderBookSnapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== OrderBookSnapshot ==");
        writeln!(f, "Order ID: {}", self.md_entry_id);
        writeln!(f, "The start time of the event processing. UNIX time in nanoseconds, according to UTC: {}", self.transact_time);
        writeln!(f, "Order price: {}", self.md_entry_px);
        writeln!(f, "Order volume: {}", self.md_entry_size);
        writeln!(f, "Trade ID: {}", self.trade_id);
        writeln!(f, "Order or trade type: {}", self.md_flags);
        writeln!(f, "Record type: {}", self.md_entry_type);
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
            md_entry_type: MDEntryType::new(parser.next::<u8>()).unwrap(),
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
        writeln!(f, "Instrument numeric code: {}", self.security_id);
        writeln!(f, "The 'MsgSeqNum' of the last message sent into incremental feed at the time of the current snapshot generation: {}", self.last_msg_seq_num_processed);
        writeln!(f, "The 'RptSeq' number of the last incremental update included in the current market data snapshot for instrument.: {}", self.rpt_seq);
        writeln!(f, "Trading session ID: {}", self.exchange_trading_session_id);
        writeln!(f, "Number of 'MDEntry' records in the current message: {}", self.no_md_entries);
        for (i, entry) in self.md_entries.iter().enumerate() {
            writeln!(f, "Entry number {}:\n{}", i, entry);
        }
        writeln!(f)
    }
}
