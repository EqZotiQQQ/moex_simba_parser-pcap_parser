use crate::packet_base::Order;
use crate::Parser;

pub struct SBEHeader {
    block_length: u16,
    template_id: u16,
    schema_id: u16,
    version: u16,
}

impl SBEHeader {
    pub fn parse(parser: &mut Parser) -> SBEHeader {
        SBEHeader {
            block_length: parser.next::<u16>(),
            template_id: parser.next::<u16>(),
            schema_id: parser.next::<u16>(),
            version: parser.next::<u16>(),
        }
    }
}

pub struct SBEMessage {
    header: SBEHeader,
    // order: Box<dyn Order>,

    parsed: u64,
}

impl SBEMessage {
    pub fn parse(parser: &mut Parser) -> SBEMessage {
        let header = SBEHeader::parse(parser);
        SBEMessage {
            header,
            parsed: 0,
        }
    }

    pub fn parsed(&self) -> u64 {
        self.parsed
    }
}


