use std::fmt::{Debug, Display, Formatter};
use crate::moex::packets::incremental_packet::IncrementalPacket;
use crate::moex::packets::market_data_packet_header::MarketDataPacketHeader;
use crate::moex::packets::snapshot_packet::SnapshotPacket;
use crate::{CustomErrors, Parser};

#[derive(Debug, Clone)]
enum PacketType {
    IncrementalPacket(IncrementalPacket),
    SnapshotPacket(SnapshotPacket),
}


#[derive(Debug, Clone)]
pub struct MarketDataPacket {
    market_data_packet_header: MarketDataPacketHeader,
    packet: PacketType,
}

impl MarketDataPacket {
    pub fn parse(parser: &mut Parser, mut length: u64) -> Result<MarketDataPacket, CustomErrors> {
        let header = MarketDataPacketHeader::parse(parser)?;

        length -= MarketDataPacketHeader::SIZE as u64; // length of market data packet header

        let packet = match header.is_incremental() {
            true => {
                let (packet, parsed) = match IncrementalPacket::parse(parser, length) {
                    Ok(r) => r,
                    Err(e) => return Err(e),
                };
                length -= parsed;
                PacketType::IncrementalPacket(packet)
            },
            false => {
                let (packet, parsed) = SnapshotPacket::parse(parser)?;
                length -= parsed;
                PacketType::SnapshotPacket(packet)
            },
        };

        parser.skip(length as usize)?;

        Ok(MarketDataPacket {
            market_data_packet_header: header,
            packet
        })
    }
}

#[allow(unused_must_use)]
impl Display for PacketType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketType::IncrementalPacket(p) => writeln!(f, "{}", p),
            PacketType::SnapshotPacket(p) => writeln!(f, "{}", p),
        }
    }
}

#[allow(unused_must_use)]
impl Display for MarketDataPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== MarketDataPacket ==");
        write!(f, "{}", self.market_data_packet_header);
        write!(f, "{}", self.packet);
        writeln!(f, "== MarketDataPacket end ==")
    }
}
