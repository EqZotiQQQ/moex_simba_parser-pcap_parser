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
    market_data_packet_header: MarketDataPacketHeader,
    packet: PacketType,
}

impl MarketDataPacket {
    pub fn parse(parser: &mut Parser, mut length: u64) -> MarketDataPacket {
        let header = MarketDataPacketHeader::parse(parser);
        // println!("MarketDataPacketHeader {}", header);

        println!("!!!{}", header);
        length -= 16; // length of market data packet header

        let packet = match header.is_incremental() {
            true => {
                let (packet, parsed) = IncrementalPacket::parse(parser, length);
                length -= parsed;
                PacketType::IncrementalPacket(packet)
            },
            false => {
                let (packet, parsed) = SnapshotPacket::parse(parser, length).unwrap();
                length -= parsed;
                PacketType::SnapshotPacket(packet)
            },
        };

        eprintln!("Skip {} bytes", length);

        parser.skip(length as usize);

        MarketDataPacket {
            market_data_packet_header: header,
            packet
        }
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
        writeln!(f, "== MarketDataPacket: ==");
        // write!(f, "Market data packet length: {}\n", self.packet_length);
        write!(f, "{}", self.market_data_packet_header);
        write!(f, "{}", self.packet);
        writeln!(f, "== MarketDataPacket end ==")
    }
}
