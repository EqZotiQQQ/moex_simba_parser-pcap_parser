use std::fmt::{Display, Formatter};
use crate::errors::CustomErrors;
use crate::moex::orders::order_best_prices::OrderBestPrices;
use crate::moex::orders::order_book_snapshot::OrderBookSnapshot;
use crate::moex::orders::order_execution::OrderExecution;
use crate::moex::orders::order_update::OrderUpdate;
use crate::moex::packets::simple_binary_encoding::sbe_header::{MessageType, SBEHeader};
use crate::Parser;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum OrderType {
    OrderUpdate(OrderUpdate),
    OrderExecution(OrderExecution),
    OrderBookSnapshot(OrderBookSnapshot),
    OrderBestPrices(OrderBestPrices),
    Heartbeat,
    SequenceReset,
    EmptyBook,
    SecurityDefinition,
    SecurityStatus,
    SecurityDefinitionUpdateReport,
    TradingSessionStatus,
    Logon,
    Logout,
    MarketDataRequest,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::OrderUpdate(order_update) => writeln!(f, "OrderUpdate: {}", order_update),
            OrderType::OrderExecution(order_execution) => writeln!(f, "OrderExecution: {}", order_execution),
            OrderType::OrderBookSnapshot(order_book_snapshot) => writeln!(f, "OrderBookSnapshot: {}", order_book_snapshot),
            OrderType::OrderBestPrices(order_best_prices) => writeln!(f, "OrderBestPrices: {}", order_best_prices),
            _ => {writeln!(f, "One of other orders")}
        }
    }
}

impl OrderType {
    fn get(&self) -> u64 {
        match self {
            OrderType::OrderUpdate(_) => OrderUpdate::SIZE as u64,
            OrderType::OrderExecution(_) => OrderExecution::SIZE as u64,
            OrderType::OrderBookSnapshot(_) => OrderBookSnapshot::SIZE as u64,
            OrderType::OrderBestPrices(_) => OrderBestPrices::SIZE as u64,
            _ => { 69 }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SBEMessage {
    header: SBEHeader,
    order: Option<OrderType>,
    parsed: u64,
}

impl SBEMessage {
    pub fn parse(parser: &mut Parser) -> Result<SBEMessage, CustomErrors> {
        let header = SBEHeader::parse(parser).unwrap();
        let mut parsed: u64 = SBEHeader::SIZE as u64;
        let mut order = match header.get_template_id() {
            MessageType::OrderBestPrices => Some(OrderType::OrderBestPrices(OrderBestPrices::parse(parser))),
            MessageType::OrderUpdate => Some(OrderType::OrderUpdate(OrderUpdate::parse(parser))),
            MessageType::OrderExecution => Some(OrderType::OrderExecution(OrderExecution::parse(parser))),
            MessageType::OrderBookSnapshot => Some(OrderType::OrderBookSnapshot(OrderBookSnapshot::parse(parser))),
            _ => {
                parser.skip(header.get_block_length() as usize); // TODO pass error
                parsed += header.get_block_length() as u64;
                None
            }
        };
        if order.is_some() {
            parsed += order.as_ref().unwrap().get();
            Ok(SBEMessage {
                header,
                order,
                parsed,
            })
        } else {
            Err(CustomErrors::BadMessageTypeError)
        }
    }

    pub fn parsed(&self) -> u64 {
        self.parsed
    }
}

#[allow(unused_must_use)]
impl Display for SBEMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== SBE message: ==");
        writeln!(f, "Version: {}", self.header);
        writeln!(f, "Order: {}", self.order.as_ref().unwrap())
    }
}


