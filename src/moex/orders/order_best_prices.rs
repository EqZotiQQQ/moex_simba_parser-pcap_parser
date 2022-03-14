use std::fmt::{Display, Formatter};
use crate::Parser;

#[derive(Debug, Clone, Copy)]
pub struct BestPricesOrderPayload {
    mkt_bid_px: i64,
    mkt_offer_px : i64,
    bp_flags : u8,
    security_id : i32,
}

impl Display for BestPricesOrderPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "mkt_bid_px: {}", self.mkt_bid_px);
        writeln!(f, "mkt_offer_px: {}", self.mkt_offer_px);
        writeln!(f, "bp_flags: {}", self.bp_flags);
        write!(f, "security_id {}", self.security_id)
    }
}

#[derive(Debug, Clone)]
pub struct OrderBestPrices {
    entry_size: u16,
    no_md_entry: u8,
    md_entries: Vec<BestPricesOrderPayload>,
}

impl Display for OrderBestPrices {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "entry_size: {}", self.entry_size);
        writeln!(f, "no_md_entry: {}", self.no_md_entry);
        for (i, entries) in self.md_entries.iter().enumerate() {
            writeln!(f, "{}", entries);
        }
        write!(f, "\n")
    }
}

impl BestPricesOrderPayload {
    pub const SIZE: u8 = 21;
    pub fn parse(parser: &mut Parser) -> BestPricesOrderPayload {
        BestPricesOrderPayload {
            mkt_bid_px: parser.next::<i64>(),
            mkt_offer_px: parser.next::<i64>(),
            bp_flags: parser.next::<u8>(),
            security_id: parser.next::<i32>(),
        }
    }
}

impl OrderBestPrices {
    pub const SIZE: u8 = 3;
    pub const TOTAL_SIZE: u8 = BestPricesOrderPayload::SIZE * OrderBestPrices::SIZE;
    pub fn parse(parser: &mut Parser) -> OrderBestPrices {
        let s = parser.next::<u16>();
        let n = parser.next::<u8>();
        OrderBestPrices {
            entry_size: s,
            no_md_entry: n,
            md_entries: (0..n).map(|i| BestPricesOrderPayload::parse(parser)).collect(),
        }
    }
}

