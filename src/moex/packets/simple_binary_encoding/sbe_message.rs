use std::fmt::{Display, Formatter};
use crate::errors::CustomErrors;
use crate::moex::orders::order_best_prices::OrderBestPrices;
use crate::moex::orders::order_book_snapshot::{OrderBookSnapshot, OrderBookSnapshotPacket};
use crate::moex::orders::order_execution::OrderExecution;
use crate::moex::orders::order_update::OrderUpdate;
use crate::moex::packets::simple_binary_encoding::sbe_header::{MessageType, SBEHeader};
use crate::Parser;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum OrderType {
    OrderUpdate(OrderUpdate),
    OrderExecution(OrderExecution),
    OrderBookSnapshotPacket(OrderBookSnapshotPacket),
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

#[allow(unused_must_use)]
impl Display for OrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Order type: ");
        match self {
            OrderType::OrderUpdate(order_update) => writeln!(f, "OrderUpdate\n{}", order_update),
            OrderType::OrderExecution(order_execution) => writeln!(f, "OrderExecution\n{}", order_execution),
            OrderType::OrderBookSnapshotPacket(order_book_snapshot) => writeln!(f, "OrderBookSnapshot\n{}", order_book_snapshot),
            OrderType::OrderBestPrices(order_best_prices) => writeln!(f, "OrderBestPrices\n{}", order_best_prices),
            _ => {writeln!(f, "One of other orders")}
        }
    }
}

#[derive(Debug, Clone)]
pub struct SBEMessage {
    header: SBEHeader,
    order: Option<OrderType>,
}

#[allow(unused_must_use)]
impl SBEMessage {
    pub fn parse(parser: &mut Parser) -> (SBEMessage, u64) {
        const SIZE: u32 = 42;
        let header = SBEHeader::parse(parser).unwrap();
        let mut parsed: u64 = SBEHeader::SIZE as u64;
        let order = match header.get_template_id() {
            MessageType::OrderBestPrices => {
                let (order, parsed_from_order) = OrderBestPrices::parse(parser);
                parsed += parsed_from_order;
                Some(OrderType::OrderBestPrices(order))
            },
            MessageType::OrderUpdate => {
                let (order, parsed_from_order) = OrderUpdate::parse(parser);
                parsed += parsed_from_order;
                Some(OrderType::OrderUpdate(order))
            },
            MessageType::OrderExecution => {
                let (order, parsed_from_order) = OrderExecution::parse(parser);
                parsed += parsed_from_order;
                Some(OrderType::OrderExecution(order))
            },
            MessageType::OrderBookSnapshotPacket => {
                let (order, parsed_from_order) = OrderBookSnapshotPacket::parse(parser);
                parsed += parsed_from_order;
                Some(OrderType::OrderBookSnapshotPacket(order))
            },
            _ => {
                parser.skip(header.get_block_length() as usize); // TODO pass error
                parsed += header.get_block_length() as u64;
                None
            }
        };
        // println!("Parsed in sbe: {}", parsed);
        (SBEMessage {
            header,
            order,
        }, parsed)
    }

}

#[allow(unused_must_use)]
impl Display for SBEMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== SBE message: ==");
        writeln!(f, "Version: {}", self.header);
        if self.order.is_some() {
            writeln!(f, "Order: {}", self.order.as_ref().unwrap())
        } else {
            writeln!(f)
        }
    }
}


