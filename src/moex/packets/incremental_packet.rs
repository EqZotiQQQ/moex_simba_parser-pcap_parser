use std::fmt::{Display, Formatter};
use crate::moex::packets::simple_binary_encoding::sbe_message::SBEMessage;
use crate::Parser;

const INCREMENTAL_PACKET_HEADER_SIZE: u8 = 12;

#[derive(Debug, Clone)]
struct IncrementalPacketHeader {
    transaction_time: u64,
    exchange_trading_session_id: u32,
}

#[allow(unused_must_use)]
impl Display for IncrementalPacketHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=================== Incremental packet header: ===================");
        write!(f, "transaction_time: {}\n", self.transaction_time);
        writeln!(f, "\nexchange_trading_session_id: {}", self.exchange_trading_session_id)
    }
}


impl IncrementalPacketHeader {
    fn parse(parser: &mut Parser) -> IncrementalPacketHeader {
        IncrementalPacketHeader {
            transaction_time: parser.next::<u64>(),
            exchange_trading_session_id: parser.next::<u32>(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IncrementalPacket {
    header: IncrementalPacketHeader,
    sbe_messages: Vec<SBEMessage>,
    size: u64,
}

#[allow(unused_must_use)]
impl Display for IncrementalPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=================== Market data packet: ===================");
        write!(f, "header: {}\n", self.header);
        for (i, msg) in self.sbe_messages.iter().enumerate() {
            write!(f, "Message number {}:\n{}", i, msg);
        }
        writeln!(f)
    }
}

impl IncrementalPacket {
    pub(crate) fn parse(parser: &mut Parser, mut size: u64) -> IncrementalPacket {
        let header = IncrementalPacketHeader::parse(parser);
        size -= INCREMENTAL_PACKET_HEADER_SIZE as u64;
        let mut sbe_messages: Vec<SBEMessage> = vec![];
        while size > 0 {
            let mut sbe_message = SBEMessage::parse(parser).unwrap();
            size -= sbe_message.parsed();
            sbe_messages.push(sbe_message);
        }
        IncrementalPacket {
            header,
            sbe_messages,
            size,
        }
    }
}
