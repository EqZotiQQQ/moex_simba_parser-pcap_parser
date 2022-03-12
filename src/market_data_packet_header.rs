use std::fmt::{Display, Formatter};
use crate::Parser;

const MESSAGE_FRAGMENTATION: u16 = 0x1;
const FIRST_MESSAGE: u16 = 0x2;
const LAST_MESSAGE: u16 = 0x4;
const INCREMENTAL_MESSAGE: u16 = 0x8;
const POS_DUP_FLAG: u16 = 0x10;

#[derive(Debug, Clone)]
pub struct MarketDataPacketHeader {
    msg_seq_number: u32,
    msg_size: u16,
    msg_flags: u16,
    sending_time: u64,
}

impl MarketDataPacketHeader {
    pub fn parse(parser: &mut Parser) -> MarketDataPacketHeader {
        MarketDataPacketHeader {
            msg_seq_number: parser.next_le::<u32>(),
            msg_size: parser.next_le::<u16>(),
            msg_flags: parser.next_le::<u16>(),
            sending_time: parser.next_le::<u64>(),
        }
    }

    pub fn is_incremental(&self) -> bool {
        (self.msg_flags & INCREMENTAL_MESSAGE) == INCREMENTAL_MESSAGE
    }
}

impl Display for MarketDataPacketHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Market data packet header: ==");
        write!(f, "Message sequential number: {}\n", self.msg_seq_number);
        write!(f, "Message size: {}\n", self.msg_size);
        write!(f, "Message flags: {}\n", self.msg_flags);
        writeln!(f, "Sending time: {}", self.sending_time)
    }
}
