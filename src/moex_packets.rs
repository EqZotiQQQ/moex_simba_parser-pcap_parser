use crate::Parser;

pub trait Packet {
    fn parse(parser: &mut Parser) -> Self;
}

struct IncrementalPacket {
    header: IncrementalPacketHeader,
    // sbe_messages: Vec<SBEMessage>,
    size: u64,
}

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
struct SnapshotPacket {

}

