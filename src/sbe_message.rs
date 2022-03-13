use std::fmt::{Display, Formatter};
use crate::errors::CustomErrors;
use crate::order_best_prices::OrderBestPrices;
use crate::order_book_snapshot::OrderBookSnapshot;
use crate::order_execution::OrderExecution;
use crate::order_update::OrderUpdate;
use crate::Parser;

#[derive(Debug, Clone)]
pub struct SBEHeader {
    block_length: u16,
    template_id: MessageType,
    schema_id: u16,
    version: u16,
}


#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Heartbeat = 1,
    SequenceReset = 2,
    OrderBestPrices = 3,
    EmptyBook = 4,
    OrderUpdate = 5,
    OrderExecution = 6,
    OrderBookSnapshot = 7,
    SecurityDefinition = 8,
    SecurityStatus = 9,
    SecurityDefinitionUpdateReport = 10,
    TradingSessionStatus = 11,
    Logon = 1000,
    Logout = 1001,
    MarketDataRequest = 1002,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl SBEHeader {
    pub fn parse(parser: &mut Parser) -> Result<SBEHeader, CustomErrors> {
        Ok(SBEHeader {
            block_length: parser.next::<u16>(),
            template_id: match parser.next::<u16>() {
                1 => MessageType::Heartbeat,
                2 => MessageType::SequenceReset,
                3 => MessageType::OrderBestPrices,
                4 => MessageType::EmptyBook,
                5 => MessageType::OrderUpdate,
                6 => MessageType::OrderExecution,
                7 => MessageType::OrderBookSnapshot,
                8 => MessageType::SecurityDefinition,
                9 => MessageType::SecurityStatus,
                10 => MessageType::SecurityDefinitionUpdateReport,
                11 => MessageType::TradingSessionStatus,
                1000 => MessageType::Logon,
                1001 => MessageType::Logout,
                1002 => MessageType::MarketDataRequest,
                _ => return Err(CustomErrors::BadMessageTypeError)
            },
            schema_id: parser.next::<u16>(),
            version: parser.next::<u16>(),
        })
    }

    pub fn get_template_id(&self) -> MessageType {
        self.template_id
    }

    pub fn get_block_length(&self) -> u16 {
        self.block_length
    }
}

#[derive(Debug, Clone)]
enum OrderType {
    OrderUpdate(OrderUpdate),
    OrderExecution(OrderExecution),
    OrderBookSnapshot(OrderBookSnapshot),
    OrderBestPrices(OrderBestPrices),
    Hearthbeat,
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

#[derive(Debug, Clone)]
pub struct SBEMessage {
    header: SBEHeader,
    order: Option<OrderType>,
    parsed: u64,
}

impl SBEMessage {
    pub fn parse(parser: &mut Parser) -> Result<SBEMessage, CustomErrors> {
        let header = SBEHeader::parse(parser).unwrap();
        let order: Option<OrderType> = match header.get_template_id() {
            MessageType::Heartbeat => None,
            MessageType::SequenceReset => None,
            MessageType::OrderBestPrices => Some(OrderType::OrderBestPrices(OrderBestPrices::parse(parser))),
            MessageType::EmptyBook => None,
            MessageType::OrderUpdate => Some(OrderType::OrderUpdate(OrderUpdate::parse(parser))),
            MessageType::OrderExecution => Some(OrderType::OrderExecution(OrderExecution::parse(parser))),
            MessageType::OrderBookSnapshot => Some(OrderType::OrderBookSnapshot(OrderBookSnapshot::parse(parser))),
            MessageType::SecurityDefinition => None,
            MessageType::SecurityStatus => None,
            MessageType::SecurityDefinitionUpdateReport => None,
            MessageType::TradingSessionStatus => None,
            MessageType::Logon => None,
            MessageType::Logout => None,
            MessageType::MarketDataRequest => None,
            _ => {
                parser.skip(header.get_block_length() as usize);
                None
            }
        };
        if order.is_some() {
            Ok(SBEMessage {
                header: header,
                order,
                parsed: 0,
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
impl Display for SBEHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== UDP header: ==");
        write!(f, "Block length: {}\n", self.block_length);
        write!(f, "Template id: {}\n", self.template_id);
        write!(f, "Schema ID: {}\n", self.schema_id);
        writeln!(f, "Version: {}", self.version)
    }
}

#[allow(unused_must_use)]
impl Display for SBEMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== SBE message: ==");
        writeln!(f, "Version: {}", self.header)
    }
}


