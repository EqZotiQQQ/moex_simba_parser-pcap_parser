use crate::Parser;

#[derive(Debug, Clone)]
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
