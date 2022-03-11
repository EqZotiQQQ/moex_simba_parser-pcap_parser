use crate::glob_pcap_header_parser::GlobalPcapHeader;
use crate::ip_header::IpHeader;
use crate::market_data_packet::MarketDataPacket;
use crate::parser::Parser;
use crate::record_header_parser::RecordHeader;
use crate::udp_header::UdpHeader;

mod glob_pcap_header_parser;
mod parser;
mod tests;
mod errors;
mod record_header_parser;
mod ip_header;
mod udp_header;
mod market_data_packet_header;
mod market_data_packet;
mod moex_packets;

fn parse() {
    let path = "sample.pcap";
    let mut parser = Parser::new(&path).expect("Failed to open file");
    let global_pcap_header = GlobalPcapHeader::parse(&mut parser).expect("Failed to parse header");
    println!("{}", global_pcap_header);

    loop {
        let record_header = RecordHeader::parse(&mut parser);
        println!("{}", record_header);

        let ip_header = IpHeader::parse(&mut parser);
        println!("{}", ip_header);

        let udp_header = UdpHeader::parse(&mut parser);
        println!("{}", udp_header);

        // MOEX SIMBA PART

        let market_data_packet = MarketDataPacket::parse(&mut parser);
        println!("{}", market_data_packet);

        break;
    }
}
