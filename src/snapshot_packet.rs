use crate::Parser;
use crate::sbe_message::SBEMessage;


struct SnapshotPacket {
    sbe_message: SBEMessage,
    size: u64,
}

impl SnapshotPacket {
    pub fn parse(parser: &mut Parser, mut length: u64) -> SnapshotPacket {
        SnapshotPacket {
            sbe_message: SBEMessage::parse(parser),
            size: length
        }
    }
}
