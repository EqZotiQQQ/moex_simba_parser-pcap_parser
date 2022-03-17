use std::fmt::{Display, Formatter};
use crate::Parser;

#[derive(Debug, Clone)]
struct MessageFlags(u16);

impl MessageFlags {
    const MESSAGE_FRAGMENTATION: u16 = 0x1;
    const FIRST_MESSAGE: u16 = 0x2;
    const LAST_MESSAGE: u16 = 0x4;
    const INCREMENTAL_MESSAGE: u16 = 0x8;
    const POS_DUP_FLAG: u16 = 0x10;
}

#[allow(unused_must_use)]
impl Display for MessageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0);
        if (self.0 & MessageFlags::MESSAGE_FRAGMENTATION) == MessageFlags::MESSAGE_FRAGMENTATION {
            writeln!(f, "* Message fragmentation");
        } else {
            writeln!(f, "* !Message fragmentation");
        }
        if (self.0 & MessageFlags::FIRST_MESSAGE) == MessageFlags::FIRST_MESSAGE {
            writeln!(f, "* First message");
        }
        if (self.0 & MessageFlags::LAST_MESSAGE) == MessageFlags::LAST_MESSAGE {
            writeln!(f, "* Last message");
        }
        if (self.0 & MessageFlags::INCREMENTAL_MESSAGE) == MessageFlags::INCREMENTAL_MESSAGE {
            writeln!(f, "* Incremental packet");
        } else {
            writeln!(f, "* !Incremental packet");
        }
        if (self.0 & MessageFlags::POS_DUP_FLAG) == MessageFlags::POS_DUP_FLAG {
            writeln!(f, "* Pos dup flag");
        } else {
            writeln!(f, "* !Pos dup flag");
        }
        writeln!(f, "== MessageFlags end ==")
    }
}

#[derive(Debug, Clone)]
pub struct MarketDataPacketHeader {
    msg_seq_number: u32,
    msg_size: u16,
    msg_flags: MessageFlags,
    sending_time: u64,
}

impl MarketDataPacketHeader {
    pub fn parse(parser: &mut Parser) -> MarketDataPacketHeader {
        MarketDataPacketHeader {
            msg_seq_number: parser.next_le::<u32>(),
            msg_size: parser.next_le::<u16>(),
            msg_flags: MessageFlags(parser.next_le::<u16>()),
            sending_time: parser.next_le::<u64>(),
        }
    }

    pub fn is_incremental(&self) -> bool {
        (self.msg_flags.0 & MessageFlags::INCREMENTAL_MESSAGE) == MessageFlags::INCREMENTAL_MESSAGE
    }
}

#[allow(unused_must_use)]
impl Display for MarketDataPacketHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Market data packet header: ==");
        write!(f, "Message sequential number: {}\n", self.msg_seq_number);
        write!(f, "Message size: {}\n", self.msg_size);
        write!(f, "Message flags: {}", self.msg_flags);
        writeln!(f, "Sending time: {}", self.sending_time);
        writeln!(f, "== MarketDataPacketHeader end ==")
    }
}

