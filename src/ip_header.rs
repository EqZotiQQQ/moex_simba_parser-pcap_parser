use std::fmt::{Display, Formatter};
use mac_address::MacAddress;
use crate::Parser;

const IP_HEADER: u16 = 14;

pub struct IpHeader {
    destination_mac: MacAddress,
    source_mac: MacAddress,
    protocol_version: u16,
    strange_field: u8,
    differentiated_services_field: u8,
    total_length: u16,
    identification: u16,
    flags_and_fragment_offset: u16,
    ttl: u8,
    udp_protocol: u8,
}

impl IpHeader {
    pub fn parse(parser: &mut Parser) -> IpHeader {
        IpHeader {
            destination_mac: MacAddress::from(parser.next_mac()),
            source_mac: MacAddress::from(parser.next_mac()),
            protocol_version: parser.next_be::<u16>(),
            strange_field: parser.next::<u8>(),
            differentiated_services_field: parser.next::<u8>(),
            total_length: parser.next_be::<u16>(),
            identification: parser.next_be::<u16>(),
            flags_and_fragment_offset: parser.next_be::<u16>(),
            ttl: parser.next::<u8>(),
            udp_protocol: parser.next::<u8>(),
        }
    }
}

impl Display for IpHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== IP header: ==");
        write!(f, "Destination mac: {}\n", self.destination_mac);
        write!(f, "Source mac: {}\n", self.source_mac);
        write!(f, "Protocol version: {}\n", self.protocol_version);
        write!(f, "Strange field (TODO): {}\n", self.strange_field);
        write!(f, "Differential services field: {}\n", self.differentiated_services_field);
        write!(f, "Total length: {}\n", self.total_length);
        write!(f, "Identification: {}\n", self.identification);
        write!(f, "Flags and fragment offset: {}\n", self.flags_and_fragment_offset);
        write!(f, "Time to live: {}\n", self.ttl);
        writeln!(f, "UDP protocol: {}", self.udp_protocol)
    }
}

