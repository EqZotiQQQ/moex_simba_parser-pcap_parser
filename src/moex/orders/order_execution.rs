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

impl Display for OrderExecution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "md_entry_id: {}", self.md_entry_id);
        writeln!(f, "md_entry_px: {}", self.md_entry_px);
        writeln!(f, "md_entry_size: {}", self.md_entry_size);
        writeln!(f, "last_px: {}", self.last_px);
        writeln!(f, "last_qty: {}", self.last_qty);
        writeln!(f, "trade_id: {}", self.trade_id);
        writeln!(f, "md_flags: {}", self.md_flags);
        writeln!(f, "security_id: {}", self.security_id);
        writeln!(f, "rpt_seq: {}", self.rpt_seq);
        writeln!(f, "md_update_action: {}", self.md_update_action);
        writeln!(f, "md_entry_type: {}", self.md_entry_type)
    }
}

impl OrderExecution {
    pub const SIZE: u8 = 66;
    pub fn parse(parser: &mut Parser) -> OrderExecution {
        OrderExecution {
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
        }
    }
}
