use std::fmt::{Display, Formatter};
use std::net::Ipv4Addr;
use mac_address::MacAddress;
use crate::Parser;

const UDP_HEADER_LENGTH: u16 = 26;

pub struct UdpHeader {
    check_sum: u16,
    source_ip: Ipv4Addr,
    dest_ip: Ipv4Addr,
    source_port: u16,
    destination_port: u16,
    length: u16,
    check_sum_udp: u16,
}

impl UdpHeader {
    pub fn parse(parser: &mut Parser) -> UdpHeader {
        UdpHeader {
            check_sum: parser.next_be::<u16>(),
            source_ip: Ipv4Addr::from(parser.next_ip_v4()),
            dest_ip: Ipv4Addr::from(parser.next_ip_v4()),
            source_port: parser.next_be::<u16>(),
            destination_port: parser.next_be::<u16>(),
            length: parser.next_be::<u16>(),
            check_sum_udp: parser.next_be::<u16>(),
        }
    }
}

impl Display for UdpHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== UDP header: ==");
        write!(f, "Check sum: {}\n", self.check_sum);
        write!(f, "Source ip: {}\n", self.source_ip);
        write!(f, "Destination ip: {}\n", self.dest_ip);
        write!(f, "Source port: {}\n", self.source_port);
        write!(f, "Destination port: {}\n", self.destination_port);
        writeln!(f, "Length: {}", self.length)
    }
}
