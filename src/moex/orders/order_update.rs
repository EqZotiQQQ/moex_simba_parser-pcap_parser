use std::fmt::{Display, Formatter};
use crate::Parser;

#[derive(Debug, Clone, Copy)]
pub struct OrderUpdate  {
    md_entry_id: i64,
    md_entry_px: i64,
    md_entry_size: i64,
    md_flags: u64,
    security_id: i32,
    rpt_seq: u32,
    md_update_action: u8,
    md_entry_type: u8,
}

impl Display for OrderUpdate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "md_entry_id: {}", self.md_entry_id);
        writeln!(f, "md_entry_px: {}", self.md_entry_px);
        writeln!(f, "md_entry_size: {}", self.md_entry_size);
        writeln!(f, "md_flags: {}", self.md_flags);
        writeln!(f, "security_id: {}", self.security_id);
        writeln!(f, "rpt_seq: {}", self.rpt_seq);
        writeln!(f, "md_update_action: {}", self.md_update_action);
        writeln!(f, "md_entry_type: {}", self.md_entry_type)
    }
}

impl OrderUpdate  {
    pub const SIZE: u8 = 42;
    pub fn parse(parser: &mut Parser) -> OrderUpdate  {
        OrderUpdate  {
            md_entry_id: parser.next::<i64>(),
            md_entry_px: parser.next::<i64>(),
            md_entry_size: parser.next::<i64>(),
            md_flags: parser.next::<u64>(),
            security_id: parser.next::<i32>(),
            rpt_seq: parser.next::<u32>(),
            md_update_action: parser.next::<u8>(),
            md_entry_type: parser.next::<u8>(),
        }
    }
}
