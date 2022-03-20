use std::fmt::{Display, Formatter};
use crate::moex::orders::details::details::{DECIMAL5_NULL, INT64_NULL, MDEntryType, MDUpdateAction};
use crate::moex::orders::order_update::EntryType;
use crate::{CustomErrors, Parser};

#[derive(Debug, Clone, Copy)]
pub struct OrderExecution {
    md_entry_id: i64,
    md_entry_px: i64,
    md_entry_size: i64,
    last_px: i64,
    last_qty : i64,
    trade_id: i64,
    md_flags: EntryType,
    security_id: i32,
    rpt_seq: u32,
    md_update_action: MDUpdateAction,
    md_entry_type: MDEntryType,
}

#[allow(unused_must_use)]
impl Display for OrderExecution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== OrderExecution ==");
        writeln!(f, "Order ID: {}", self.md_entry_id);
        if self.md_entry_px == DECIMAL5_NULL {
            writeln!(f, "Order price: Null");
        } else {
            writeln!(f, "Order price: {}", self.md_entry_px);
        }
        if self.md_entry_size == INT64_NULL {
            writeln!(f, "Remaining quantity in the order: Null");
        } else {
            writeln!(f, "Remaining quantity in the order: {}", self.md_entry_size);
        }
        writeln!(f, "Trade price: {}", self.last_px);
        writeln!(f, "Trade volume: {}", self.last_qty);
        writeln!(f, "Trade ID: {}", self.trade_id);
        writeln!(f, "Trade type (Flags): {}", self.md_flags);
        writeln!(f, "Instrument numeric code: {}", self.security_id);
        writeln!(f, "Incremental refresh sequence number: {}", self.rpt_seq);
        writeln!(f, "Incremental refresh type: {}", self.md_update_action);
        writeln!(f, "Record type: {}", self.md_entry_type);
        writeln!(f, "== OrderExecution end ==")
    }
}

impl OrderExecution {
    pub const SIZE: u8 = 66;
    pub fn parse(parser: &mut Parser) -> Result<(OrderExecution, u64), CustomErrors> {
        Ok((OrderExecution {
            md_entry_id: parser.next::<i64>()?,
            md_entry_px: parser.next::<i64>()?,
            md_entry_size: parser.next::<i64>()?,
            last_px: parser.next::<i64>()?,
            last_qty: parser.next::<i64>()?,
            trade_id: parser.next::<i64>()?,
            md_flags: EntryType(parser.next::<i64>()? as u64),
            security_id: parser.next::<i32>()?,
            rpt_seq: parser.next::<u32>()?,
            md_update_action: MDUpdateAction::new(parser.next::<u8>()?)?,
            md_entry_type: MDEntryType::new(parser.next::<u8>()?)?,
        }, OrderExecution::SIZE as u64))
    }
}
