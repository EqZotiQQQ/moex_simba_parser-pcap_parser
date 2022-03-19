use std::fmt::{Display, Formatter};
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
            ProtocolVersion::IPv4(v) => write!(f, "IPv4 ({})", v)
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
            Protocol::Udp(v) => write!(f, "UDP ({})", v)
        }
    }
}

struct FragmentAndOffset(u16);

#[allow(unused_must_use)]
impl Display for FragmentAndOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f);
        match self.get_reserve_bit() {
            Bit::Set => writeln!(f, "* Reserve bit set"),
            Bit::NotSet => writeln!(f, "* Reserve bit unset"),
        };
        match self.get_dont_fragment_bit() {
            Bit::Set => writeln!(f, "* Don't fragment bit set"),
            Bit::NotSet => writeln!(f, "* Don't fragment bit unset"),
        };
        match self.get_more_fragments_bit() {
            Bit::Set => writeln!(f, "* More fragments bit set"),
            Bit::NotSet => writeln!(f, "* More fragments bit unset"),
        };
        write!(f, "* Length bit: {}", self.get_length_bit())
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

    pub fn get_reserve_bit(&self) -> Bit {
        if ((self.0 >> 1) & 1) == 1 {
            Bit::Set
        } else {
            Bit::NotSet
        }
    }

    pub fn get_dont_fragment_bit(&self) -> Bit {
        if ((self.0 >> 2) & 1) == 1 {
            Bit::NotSet
        } else {
            Bit::Set
        }
    }

    pub fn get_more_fragments_bit(&self) -> Bit {
        if ((self.0 >> 3) & 1) == 1 {
            Bit::Set
        } else {
            Bit::NotSet
        }
    }

    pub fn get_length_bit(&self) -> u16 {
        self.0 & 0x023C
    }
}

struct VersionAndLength(u8);

enum Version {
    IPv4,
    IPv6,
}

enum DifferentialServiceCodePoint {
    CS0,
    // CS1, Unsupported
    // CS2,
    // CS3,
    // CS4,
    // CS5,
    // CS6,
    // CS7,
}

impl DifferentialServiceCodePoint {
    pub fn new(field: u8) -> Result<DifferentialServiceCodePoint, CustomErrors> {
        Ok(match field {
            0 => DifferentialServiceCodePoint::CS0,
            _ => return Err(CustomErrors::UnsupportedDifferentialServiceCodePoint),
        })
    }
}

impl Display for DifferentialServiceCodePoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DifferentialServiceCodePoint::CS0 => write!(f, "CS0")
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::IPv4 => write!(f, "IPv4"),
            Version::IPv6 => write!(f, "IPv6"),
        }
    }
}

impl Display for VersionAndLength {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "* Version: {}", self.get_version());
        write!(f, "* Length: {}", self.get_header_length())
    }
}

impl VersionAndLength {
    pub fn get_version(&self) -> Version {
        if (self.0 & 0x04) == 0x04 {
            Version::IPv4
        } else {
            Version::IPv6
        }
    }

    pub fn get_header_length(&self) -> u8 {
        if (self.0 & 0x05) == 0x05 {
            20
        } else {
            0 // 0 isn't valid, minimum is 20 bytes in another branch. TODO fill this field correctly
        }
    }
}

pub struct IpHeader {
    destination_mac: MacAddress,
    source_mac: MacAddress,
    protocol_version: ProtocolVersion,
    version_and_length: VersionAndLength,
    differentiated_services_field: DifferentialServiceCodePoint,
    total_length: u16,
    identification: u16,
    flags_and_fragment_offset: FragmentAndOffset,
    ttl: u8,
    udp_protocol: Protocol,
}

impl IpHeader {
    pub const SIZE: u8 = 16;
    pub fn parse(parser: &mut Parser) -> Result<IpHeader, CustomErrors> {
        let destination_mac = MacAddress::from(parser.next_mac());
        let source_mac = MacAddress::from(parser.next_mac());
        let protocol_version = ProtocolVersion::new(parser.next_be::<u16>())?;
        let version_and_length = VersionAndLength(parser.next::<u8>());
        let differentiated_services_field = DifferentialServiceCodePoint::new(parser.next::<u8>())?;
        let total_length = parser.next_be::<u16>();
        let identification = parser.next_be::<u16>();
        let flags_and_fragment_offset = FragmentAndOffset::new(parser.next_be::<u16>());
        let ttl = parser.next::<u8>();
        let udp_protocol = Protocol::new(parser.next::<u8>())?;
        Ok(IpHeader {
            destination_mac,
            source_mac,
            protocol_version,
            version_and_length,
            differentiated_services_field,
            total_length,
            identification,
            flags_and_fragment_offset,
            ttl,
            udp_protocol,
        })
    }
}

#[allow(unused_must_use)]
impl Display for IpHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== IP header: ==");
        writeln!(f, "Destination mac: {}", self.destination_mac);
        writeln!(f, "Source mac: {}", self.source_mac);
        writeln!(f, "Protocol version: {}", self.protocol_version);
        writeln!(f, "Version and internet header length [4 bits | 4 bits]:\n{}", self.version_and_length);
        writeln!(f, "Differential services field: {}", self.differentiated_services_field);
        writeln!(f, "Total packet length is {} bytes. (from 20 bytes to 65535 bytes)", self.total_length);
        writeln!(f, "Identification: {:#02X}", self.identification);
        writeln!(f, "Flags and fragment offset: [3 bits | 13 bits]{}", self.flags_and_fragment_offset);
        writeln!(f, "Time to live: {}", self.ttl);
        writeln!(f, "UDP protocol: {}", self.udp_protocol);
        writeln!(f, "== IpHeader end ==")
    }
}

