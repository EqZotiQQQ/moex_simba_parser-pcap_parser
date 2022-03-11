use std::fmt::{Display, Formatter};
use crate::Parser;

pub struct MarketDataPacketHeader {
    msg_seq_number: u32,
    msg_size: u16,
    msg_flags: u16,
    sending_time: u64,
}

impl MarketDataPacketHeader {
    pub fn parse(parser: &mut Parser) -> MarketDataPacketHeader {
        MarketDataPacketHeader {
            msg_seq_number: parser.next::<u32>(),
            msg_size: parser.next::<u16>(),
            msg_flags: parser.next::<u16>(),
            sending_time: parser.next::<u64>(),
        }
    }
}

impl Display for MarketDataPacketHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message sequential number: {}\n", self.msg_seq_number);
        write!(f, "Message size: {}\n", self.msg_size);
        write!(f, "Message flags: {}\n", self.msg_flags);
        writeln!(f, "Sending time: {}", self.sending_time)
    }
}