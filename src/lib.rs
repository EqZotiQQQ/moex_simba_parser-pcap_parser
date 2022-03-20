use crate::errors::CustomErrors;
use crate::glob_pcap_header_parser::GlobalPcapHeader;
use crate::record_header_parser::RecordHeader;
use crate::ip_header::IpHeader;
use crate::udp_header::UdpHeader;
use crate::parser::Parser;
use crate::moex::packets::market_data_packet::MarketDataPacket;

mod glob_pcap_header_parser;
mod parser;
mod tests;
mod errors;
mod record_header_parser;
mod ip_header;
mod udp_header;
mod moex;
mod utils;

pub fn parse() -> Result<u64, CustomErrors> {
    let path = "sample.pcap";
    let mut parser = Parser::new(&path)?;
    let global_pcap_header = GlobalPcapHeader::parse(&mut parser)?;

    println!("{}", global_pcap_header);

    for i in 1..10 {
        println!("\nPacket number {}\n\n", i);

        let record_header = RecordHeader::parse(&mut parser)?;

        println!("{}", record_header);

        let mut len = record_header.get_packet_len() as u64;

        let ip_header = IpHeader::parse(&mut parser)?;

        println!("{}", ip_header);
        len -= IpHeader::SIZE as u64; // ip header size

        let udp_header = UdpHeader::parse(&mut parser)?;

        println!("{}", udp_header);

        len -= UdpHeader::SIZE as u64; // udp header size
        // MOEX SIMBA PART

        let market_data_packet = MarketDataPacket::parse(&mut parser, len)?;

        println!("{}", market_data_packet);
    }
    Ok(42 as u64)
}
