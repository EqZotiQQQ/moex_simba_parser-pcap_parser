use std::fmt::{Display, Formatter};
use crate::moex::packets::simple_binary_encoding::sbe_message::SBEMessage;
use crate::{CustomErrors, Parser};

#[derive(Debug, Clone)]
struct IncrementalPacketHeader {
    transaction_time: u64,
    exchange_trading_session_id: u32,
}

#[allow(unused_must_use)]
impl Display for IncrementalPacketHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Incremental packet header ==");
        write!(f, "Transaction time: {}\n", self.transaction_time);
        writeln!(f, "Exchange trading session ID: {}", self.exchange_trading_session_id);
        writeln!(f, "== Incremental packet header end ==")
    }
}

impl IncrementalPacketHeader {
    pub const SIZE: u8 = 12;
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
}

#[allow(unused_must_use)]
impl Display for IncrementalPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Incremental packet ==");
        write!(f, "{}\n", self.header);
        for (i, msg) in self.sbe_messages.iter().enumerate() {
            write!(f, "Message number {}/{}:\n\n{}", i + 1, self.sbe_messages.len() , msg);
        }
        write!(f, "== Incremental packet end ==")
    }
}

impl IncrementalPacket {
    pub fn parse(parser: &mut Parser, size: u64) -> Result<(IncrementalPacket, u64), CustomErrors> {
        let header = IncrementalPacketHeader::parse(parser);
        let mut parsed = 0;
        parsed += IncrementalPacketHeader::SIZE as u64;
        let mut sbe_messages: Vec<SBEMessage> = vec![];
        while size > parsed {
            let (sbe_message, parsed_from_sbe) = match SBEMessage::parse(parser) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            parsed += parsed_from_sbe;
            sbe_messages.push(sbe_message);
        }
        Ok((IncrementalPacket {
            header,
            sbe_messages,
        }, size))
    }
}
