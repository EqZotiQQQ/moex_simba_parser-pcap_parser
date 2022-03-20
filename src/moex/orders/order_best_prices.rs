use std::fmt::{Display, Formatter};
use crate::{CustomErrors, Parser};

#[allow(unused_must_use)]
#[derive(Debug, Clone, Copy)]
pub struct BestPricesOrderPayload {
    mkt_bid_px: i64,
    mkt_offer_px : i64,
    bp_flags : u8,
    security_id : i32,
}

#[allow(unused_must_use)]
impl Display for BestPricesOrderPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== BestPricesOrderPayload ==");
        writeln!(f, "mkt_bid_px: {}", self.mkt_bid_px);
        writeln!(f, "mkt_offer_px: {}", self.mkt_offer_px);
        writeln!(f, "bp_flags: {}", self.bp_flags);
        write!(f, "security_id {}", self.security_id);
        writeln!(f, "== BestPricesOrderPayload end ==")
    }
}

#[derive(Debug, Clone)]
pub struct OrderBestPrices {
    entry_size: u16,
    no_md_entry: u8,
    md_entries: Vec<BestPricesOrderPayload>,
}

#[allow(unused_must_use)]
impl Display for OrderBestPrices {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== OrderBestPrices ==");
        writeln!(f, "entry_size: {}", self.entry_size);
        writeln!(f, "no_md_entry: {}", self.no_md_entry);
        for (i, entries) in self.md_entries.iter().enumerate() {
            writeln!(f, "Best price no {}:\n {}", i, entries);
        }
        writeln!(f, "== OrderBestPrices end ==")
    }
}

impl BestPricesOrderPayload {
    pub const SIZE: u8 = 21;
    pub fn parse(parser: &mut Parser) -> Result<BestPricesOrderPayload, CustomErrors> {
        Ok(BestPricesOrderPayload {
            mkt_bid_px: parser.next::<i64>()?,
            mkt_offer_px: parser.next::<i64>()?,
            bp_flags: parser.next::<u8>()?,
            security_id: parser.next::<i32>()?,
        })
    }
}

impl OrderBestPrices {
    pub const SIZE: u8 = 3;
    pub const TOTAL_SIZE: u8 = BestPricesOrderPayload::SIZE * OrderBestPrices::SIZE;
    pub fn parse(parser: &mut Parser) -> Result<(OrderBestPrices, u64), CustomErrors> {
        let s = parser.next::<u16>()?;
        let n = parser.next::<u8>()?;
        let mut md_entries: Vec<BestPricesOrderPayload> = vec![];
        for _ in 0..n {
            md_entries.push(BestPricesOrderPayload::parse(parser)?);
        }
        Ok((OrderBestPrices {
            entry_size: s,
            no_md_entry: n,
            md_entries
        }, OrderBestPrices::TOTAL_SIZE as u64))
    }
}

