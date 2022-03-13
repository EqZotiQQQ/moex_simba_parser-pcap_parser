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
mod snapshot_packet;
mod market_data_packet_header;
mod market_data_packet;
mod incremental_packet;
mod sbe_message;
mod order_update;
mod order_best_prices;
mod order_book_snapshot;
mod order_execution;

fn parse() {
    let path = "sample.pcap";
    let mut parser = Parser::new(&path).expect("Failed to open file");
    let global_pcap_header = GlobalPcapHeader::parse(&mut parser).expect("Failed to parse header");
    println!("{}", global_pcap_header);

    loop {
        let record_header = RecordHeader::parse(&mut parser);
        println!("{}", record_header);

        let mut len = record_header.get_packet_len() as u64;

        let ip_header = IpHeader::parse(&mut parser);
        println!("{}", ip_header);

        len -= 16; // ip header size

        let udp_header = UdpHeader::parse(&mut parser);
        println!("{}", udp_header);

        len -= 26; // udp header size
        // MOEX SIMBA PART


        let market_data_packet = MarketDataPacket::parse(&mut parser, len);
        println!("{}", market_data_packet);

        break;
    }
}
