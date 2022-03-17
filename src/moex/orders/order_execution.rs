use std::fmt::{Display, Formatter};
use crate::Parser;

#[derive(Debug, Clone, Copy)]
pub struct OrderExecution {
    md_entry_id: i64,
    md_entry_px: i64,
    md_entry_size: i64,
    last_px: i64,
    last_qty : i64,
    trade_id: i64,
    md_flags: i64,
    security_id: i32,
    rpt_seq: u32,
    md_update_action: u8,
    md_entry_type: u8,
}

#[allow(unused_must_use)]
impl Display for OrderExecution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== OrderExecution ==");
        writeln!(f, "MD Entry Id: {}", self.md_entry_id);
        writeln!(f, "MD Entry Price: {}", self.md_entry_px);
        writeln!(f, "MD Entry Size: {}", self.md_entry_size);
        writeln!(f, "Last Px: {}", self.last_px);
        writeln!(f, "Last Qty: {}", self.last_qty);
        writeln!(f, "Trade ID: {}", self.trade_id);
        writeln!(f, "MD flags: {}", self.md_flags);
        writeln!(f, "Security ID: {}", self.security_id);
        writeln!(f, "Rpt Seq: {}", self.rpt_seq);
        writeln!(f, "MD Update action: {}", self.md_update_action);
        writeln!(f, "MD entry type: {}", self.md_entry_type);
        writeln!(f, "== OrderExecution end ==")
    }
}

impl OrderExecution {
    pub const SIZE: u8 = 66;
    pub fn parse(parser: &mut Parser) -> (OrderExecution, u64) {
        (OrderExecution {
            md_entry_id: parser.next::<i64>(),
            md_entry_px: parser.next::<i64>(),
            md_entry_size: parser.next::<i64>(),
            last_px: parser.next::<i64>(),
            last_qty: parser.next::<i64>(),
            trade_id: parser.next::<i64>(),
            md_flags: parser.next::<i64>(),
            security_id: parser.next::<i32>(),
            rpt_seq: parser.next::<u32>(),
            md_update_action: parser.next::<u8>(),
            md_entry_type: parser.next::<u8>(),
        }, OrderExecution::SIZE as u64)
    }
}
