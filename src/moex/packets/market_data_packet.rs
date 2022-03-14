use std::fmt::{Debug, Display, Formatter};
use crate::moex::packets::incremental_packet::IncrementalPacket;
use crate::moex::packets::market_data_packet_header::MarketDataPacketHeader;
use crate::moex::packets::snapshot_packet::SnapshotPacket;
use crate::Parser;

#[derive(Debug, Clone)]
enum PacketType {
    IncrementalPacket(IncrementalPacket),
    SnapshotPacket(SnapshotPacket),
}

#[derive(Debug, Clone)]
pub struct MarketDataPacket {
    packet_length: u64,
    market_data_packet_header: MarketDataPacketHeader,
    packet: PacketType,
}

impl MarketDataPacket {
    pub fn parse(parser: &mut Parser, mut length: u64) -> MarketDataPacket {
        let header = MarketDataPacketHeader::parse(parser);

        length -= 16; // length of market data packet header

        let packet = match header.is_incremental() {
            true => PacketType::IncrementalPacket(IncrementalPacket::parse(parser, length)),
            false => PacketType::SnapshotPacket(SnapshotPacket::parse(parser, length).unwrap()),
        };

        MarketDataPacket {
            packet_length: parser.next::<u64>(),
            market_data_packet_header: header,
            packet
        }
    }
}

#[allow(unused_must_use)]
impl Display for PacketType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Packet: ==");
        match self {
            PacketType::IncrementalPacket(p) => writeln!(f, "Incremental packet:\n{}", p),//IncrementalPacket::fmt(f)),
            PacketType::SnapshotPacket(p) => writeln!(f, "Snapshot packet:\n{}", p),
        }
        // writeln!(f, "\nPacket display:\n{}", self)
    }
}

#[allow(unused_must_use)]
impl Display for MarketDataPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Market data packet: ==");
        write!(f, "Packet length: {}\n", self.packet_length);
        write!(f, "\nMarket data packet header:\n{}", self.market_data_packet_header);
        writeln!(f, "\nMarket data packet header:\n{}", self.packet)
    }
}
