use std::fmt::{Display, Formatter};
use crate::market_data_packet_header::MarketDataPacketHeader;
use crate::Parser;
use crate::moex_packets;
use crate::moex_packets::Packet;

pub struct MarketDataPacket {
    packet_length: u64,
    market_data_packet_header: MarketDataPacketHeader,
    // packet: Option<Box<dyn Packet>>,
    // incremental_packet: Option<IncrementalPacket>,
    // snapshot_packet: Option<SnapshotPacket>,
}

impl MarketDataPacket {
    pub fn parse(parser: &mut Parser) -> MarketDataPacket {
        let header = MarketDataPacketHeader::parse(parser);

        MarketDataPacket {
            packet_length: parser.next::<u64>(),
            market_data_packet_header: header,
            // packet
        }
    }
}

impl Display for MarketDataPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Packet length: {}\n", self.packet_length);
        writeln!(f, "Market data packet header: {}", self.market_data_packet_header)
    }
}