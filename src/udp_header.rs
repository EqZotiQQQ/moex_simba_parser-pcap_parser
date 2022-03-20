use std::fmt::{Display, Formatter};
use std::net::Ipv4Addr;
use crate::{CustomErrors, Parser};


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
    pub const SIZE: u16 = 26;
    pub fn parse(parser: &mut Parser) -> Result<UdpHeader, CustomErrors> {
        Ok(UdpHeader {
            check_sum: parser.next_be::<u16>()?,
            source_ip: Ipv4Addr::from(parser.next_ip_v4()?),
            dest_ip: Ipv4Addr::from(parser.next_ip_v4()?),
            source_port: parser.next_be::<u16>()?,
            destination_port: parser.next_be::<u16>()?,
            length: parser.next_be::<u16>()?,
            check_sum_udp: parser.next_be::<u16>()?,
        })
    }
}

#[allow(unused_must_use)]
impl Display for UdpHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== UdpHeader: ==");
        write!(f, "Check sum: {}\n", self.check_sum);
        write!(f, "Source ip: {}:{}\n", self.source_ip, self.source_port);
        write!(f, "Destination ip: {}:{}\n", self.dest_ip,  self.destination_port);
        write!(f, "Length: {} bytes\n", self.length);
        writeln!(f, "Check sum UDP: {}", self.check_sum_udp);
        writeln!(f, "== UdpHeader end ==")
    }
}
