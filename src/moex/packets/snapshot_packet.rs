use std::fmt::{Display, Formatter};
use crate::errors::CustomErrors;
use crate::moex::packets::simple_binary_encoding::sbe_message::SBEMessage;
use crate::Parser;


#[allow(unused_must_use)]
#[derive(Debug, Clone)]
pub struct SnapshotPacket {
    sbe_message: SBEMessage
}

impl SnapshotPacket {
    pub fn parse(parser: &mut Parser, length: u64) -> Result<(SnapshotPacket, u64), CustomErrors> {
        match SBEMessage::parse(parser) {
            Ok((sbe_message, parsed_from_sbe)) => Ok((SnapshotPacket {
                sbe_message
            }, parsed_from_sbe)),
            Err(e) => Err(e)
        }
    }
}

#[allow(unused_must_use)]
impl Display for SnapshotPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Snapshot packet ==");
        write!(f, "{}", self.sbe_message);
        writeln!(f, "== SnapshotPacket end ==")
    }
}
