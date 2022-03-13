use crate::errors::CustomErrors;
use crate::Parser;
use crate::sbe_message::SBEMessage;


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
