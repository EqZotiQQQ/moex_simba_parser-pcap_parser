use std::fmt::{Display, Formatter, write};
use mac_address::MacAddress;
use crate::errors::CustomErrors;
use crate::Parser;

enum ProtocolVersion {
    IPv4(u16),
    /*Atm unsupported*/
}

impl ProtocolVersion {
    fn new(protocol_version: u16) -> Result<ProtocolVersion, CustomErrors> {
        Ok(match protocol_version {
            0x0800 => ProtocolVersion::IPv4(0x0800),
            _ => return Err(CustomErrors::UnsupportedProtocolVersion),
        })
    }
}

impl Display for ProtocolVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProtocolVersion::IPv4(v) => writeln!(f, "IPv4 ({})", v)
        }
    }
}

enum Protocol {
    Udp(u8),
    /**/
}

impl Protocol {
    fn new(protocol_version: u8) -> Result<Protocol, CustomErrors> {
        Ok(match protocol_version {
            17 => Protocol::Udp(17),
            _ => return Err(CustomErrors::UnsupportedProtocol),
        })
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Protocol::Udp(v) => writeln!(f, "UDP ({})", v)
        }
    }
}

struct FragmentAndOffset(u16);

impl Display for FragmentAndOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match FragmentAndOffset::get_reserve_bit(self.0) {
            Bit::Set => writeln!(f, "* Reserve bit set"),
            Bit::NotSet => writeln!(f, "* Reserve bit unset"),
        };
        match FragmentAndOffset::get_dont_fragment_bit(self.0) {
            Bit::Set => writeln!(f, "* Don't fragment bit set"),
            Bit::NotSet => writeln!(f, "* Don't fragment bit unset"),
        };
        match FragmentAndOffset::get_more_fragments_bit(self.0) {
            Bit::Set => writeln!(f, "* More fragments bit set"),
            Bit::NotSet => writeln!(f, "* More fragments bit unset"),
        };
        writeln!(f, "* Length bit: {}", FragmentAndOffset::get_length_bit(self.0))
    }
}
enum Bit {
    Set,
    NotSet,
}

impl FragmentAndOffset {
    fn new(fragm_and_offset: u16) -> FragmentAndOffset {
        FragmentAndOffset {
            0: fragm_and_offset
        }
    }

    pub fn get_reserve_bit(byte: u16) -> Bit {
        if ((byte >> 1) & 1) == 1 {
            Bit::Set
        } else {
            Bit::NotSet
        }
    }

    pub fn get_dont_fragment_bit(byte: u16) -> Bit {
        if ((byte >> 2) & 1) == 1 {
            Bit::NotSet
        } else {
            Bit::Set
        }
    }

    pub fn get_more_fragments_bit(byte: u16) -> Bit {
        if ((byte >> 3) & 1) == 1 {
            Bit::Set
        } else {
            Bit::NotSet
        }
    }

    pub fn get_length_bit(byte: u16) -> u16 {
        byte & 0x023C
    }
}

pub struct IpHeader {
    destination_mac: MacAddress,
    source_mac: MacAddress,
    protocol_version: ProtocolVersion,
    strange_field: u8,
    differentiated_services_field: u8,
    total_length: u16,
    identification: u16,
    flags_and_fragment_offset: FragmentAndOffset,
    ttl: u8,
    udp_protocol: Protocol,
}

impl IpHeader {
    pub const SIZE: u8 = 16;
    pub fn parse(parser: &mut Parser) -> IpHeader {
        IpHeader {
            destination_mac: MacAddress::from(parser.next_mac()),
            source_mac: MacAddress::from(parser.next_mac()),
            protocol_version: ProtocolVersion::new(parser.next_be::<u16>()).unwrap(),
            strange_field: parser.next::<u8>(),
            differentiated_services_field: parser.next::<u8>(),
            total_length: parser.next_be::<u16>(),
            identification: parser.next_be::<u16>(),
            flags_and_fragment_offset: FragmentAndOffset::new(parser.next_be::<u16>()),
            ttl: parser.next::<u8>(),
            udp_protocol: Protocol::new(parser.next::<u8>()).unwrap(),
        }
    }
}

#[allow(unused_must_use)]
impl Display for IpHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== IP header: ==");
        write!(f, "Destination mac: {}\n", self.destination_mac);
        write!(f, "Source mac: {}\n", self.source_mac);
        write!(f, "Protocol version: {}", self.protocol_version);
        write!(f, "Strange field (TODO): {}\n", self.strange_field);
        write!(f, "Differential services field: {}\n", self.differentiated_services_field);
        write!(f, "Total length: {}\n", self.total_length);
        write!(f, "Identification: {}\n", self.identification);
        write!(f, "Flags and fragment offset:\n{}", self.flags_and_fragment_offset);
        write!(f, "Time to live: {}\n", self.ttl);
        writeln!(f, "UDP protocol: {}", self.udp_protocol)
    }
}

