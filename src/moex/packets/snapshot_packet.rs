use std::fmt::{Display, Formatter};
use crate::errors::CustomErrors;
use crate::moex::packets::simple_binary_encoding::sbe_message::SBEMessage;
use crate::Parser;


#[derive(Debug, Clone)]
pub struct SnapshotPacket {
    sbe_message: SBEMessage,
    size: u64,
}

impl SnapshotPacket {
    pub fn parse(parser: &mut Parser, mut length: u64) -> Result<SnapshotPacket, CustomErrors> {
        Ok(SnapshotPacket {
            sbe_message: match SBEMessage::parse(parser) {
                Ok(packet) => packet,
                Err(e) => return Err(e),
            },
            size: length
        })
    }
}

impl Display for SnapshotPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Snapshot packet ==");
        write!(f, "{}", self.sbe_message)
    }
}
