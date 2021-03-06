use std::fmt::{Display, Formatter};
use crate::errors::CustomErrors;
use crate::Parser;

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Heartbeat = 1,
    SequenceReset = 2,
    OrderBestPrices = 3,
    EmptyBook = 4,
    OrderUpdate = 5,
    OrderExecution = 6,
    OrderBookSnapshotPacket = 7,
    SecurityDefinition = 8,
    SecurityStatus = 9,
    SecurityDefinitionUpdateReport = 10,
    TradingSessionStatus = 11,
    Logon = 1000,
    Logout = 1001,
    MarketDataRequest = 1002,
}

#[derive(Debug, Clone)]
pub struct SBEHeader {
    block_length: u16,
    template_id: MessageType,
    schema_id: u16,
    version: u16,
}

impl SBEHeader {
    pub const SIZE: u16 = 8;

    pub fn parse(parser: &mut Parser) -> Result<SBEHeader, CustomErrors> {
        let block_length = parser.next::<u16>()?;
        let template_id = match parser.next::<u16>()? {
            1 => MessageType::Heartbeat,
            2 => MessageType::SequenceReset,
            3 => MessageType::OrderBestPrices,
            4 => MessageType::EmptyBook,
            5 => MessageType::OrderUpdate,
            6 => MessageType::OrderExecution,
            7 => MessageType::OrderBookSnapshotPacket,
            8 => MessageType::SecurityDefinition,
            9 => MessageType::SecurityStatus,
            10 => MessageType::SecurityDefinitionUpdateReport,
            11 => MessageType::TradingSessionStatus,
            1000 => MessageType::Logon,
            1001 => MessageType::Logout,
            1002 => MessageType::MarketDataRequest,
            e => {
                println!("Bad msg type {}", e);
                return Err(CustomErrors::BadMessageTypeError);
            }
        };
        let schema_id = parser.next::<u16>()?;
        let version = parser.next::<u16>()?;
        Ok(SBEHeader {
            block_length,
            template_id,
            schema_id,
            version
        })
    }

    pub fn get_template_id(&self) -> MessageType {
        self.template_id
    }

    pub fn get_block_length(&self) -> u16 {
        self.block_length
    }
}

#[allow(unused_must_use)]
impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::OrderUpdate => write!(f, "(OrderUpdate)"),
            MessageType::OrderExecution => write!(f, "(OrderExecution)"),
            MessageType::OrderBookSnapshotPacket => write!(f, "(OrderBookSnapshot)"),
            MessageType::OrderBestPrices => write!(f, "(OrderBestPrices)"),
            MessageType::Heartbeat => write!(f, "(Heartbeat)"),
            MessageType::SequenceReset => write!(f, "(SequenceReset)"),
            MessageType::EmptyBook => write!(f, "(EmptyBook)"),
            MessageType::SecurityDefinition => write!(f, "(SecurityDefinition)"),
            MessageType::SecurityStatus => write!(f, "(SecurityStatus)"),
            MessageType::SecurityDefinitionUpdateReport => write!(f, "(SecurityDefinitionUpdateReport)"),
            MessageType::TradingSessionStatus => write!(f, "(TradingSessionStatus)"),
            MessageType::Logon => write!(f, "(Logon)"),
            MessageType::Logout => write!(f, "(Logout)"),
            MessageType::MarketDataRequest => write!(f, "(MarketDataRequest)"),
        }
    }
}

#[allow(unused_must_use)]
impl Display for SBEHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== SBE header ==");
        write!(f, "Block length: {}\n", self.block_length);
        write!(f, "Template id: {}\n", self.template_id);
        write!(f, "Schema ID: {}\n", self.schema_id);
        writeln!(f, "Version: {}", self.version);
        writeln!(f, "== SBE header end ==")
    }
}
