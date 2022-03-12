use std::fmt::{Display, Formatter};
use crate::incremental_packet::IncrementalPacket;
use crate::market_data_packet_header::MarketDataPacketHeader;
use crate::packet_base::Packet;
use crate::Parser;

#[derive(Debug, Clone)]
pub struct MarketDataPacket {
    packet_length: u64,
    market_data_packet_header: MarketDataPacketHeader,
    // packet: Option<Box<dyn Packet>>,
}

impl MarketDataPacket {
    pub fn parse(parser: &mut Parser, mut length: u32) -> MarketDataPacket {
        let header = MarketDataPacketHeader::parse(parser);
        // let mut p: Option<Box<dyn Packet>> = None;
        length -= 16; // length of market data packet header
        if header.is_incremental() {
            // p = Some(Box::new(IncrementalPacket::parse(parser, length)));
        }

        MarketDataPacket {
            packet_length: parser.next::<u64>(),
            market_data_packet_header: header,
            // packet: None,
        }
    }
}

impl Display for MarketDataPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Market data packet: ==");
        write!(f, "Packet length: {}\n", self.packet_length);
        writeln!(f, "Market data packet header: {}", self.market_data_packet_header)
    }
}
