use crate::packet_base::Packet;
use crate::Parser;
use crate::sbe_message::SBEMessage;

const INCREMENTAL_PACKET_HEADER_SIZE: u8 = 12;

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

pub struct IncrementalPacket {
    header: IncrementalPacketHeader,
    sbe_messages: Vec<SBEMessage>,
    size: u64,
}

impl Packet for IncrementalPacket {
    fn parse(parser: &mut Parser, mut size: u64) -> IncrementalPacket {
        let header = IncrementalPacketHeader::parse(parser);
        size -= INCREMENTAL_PACKET_HEADER_SIZE as u64;
        let mut sbe_messages: Vec<SBEMessage> = vec![];
        while size > 0 {
            let sbe_message = SBEMessage::parse(parser);
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


