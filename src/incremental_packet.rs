use crate::Parser;
use crate::sbe_message::SBEMessage;

const INCREMENTAL_PACKET_HEADER_SIZE: u8 = 12;

#[derive(Debug, Clone)]
struct IncrementalPacketHeader {
    transaction_time: u64,
    exchange_trading_session_id: u32,
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


