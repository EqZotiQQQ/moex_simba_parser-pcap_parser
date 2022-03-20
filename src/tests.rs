#[cfg(test)]
mod tests {
    use crate::glob_pcap_header::GlobalPcapHeader;
    use crate::parser::Parser;

    #[test]
    fn parse_u8() {
        let path = "00_to_ff.bin";
        let mut parser = Parser::new(path)?;
        assert_eq!(parser.next::<u8>(), 0);
        assert_eq!(parser.next::<u8>(), 1);
        assert_eq!(parser.next::<u8>(), 2);
    }

    #[test]
    fn parse_u16() {
        let path = "00_to_ff.bin";
        let mut parser = Parser::new(path)?;
        assert_eq!(parser.next::<u16>(), 1);
        assert_eq!(parser.next::<u16>(), 515);
        assert_eq!(parser.next::<u16>(), 1029);
    }

    #[test]
    fn parse_u32() {
        let path = "00_to_ff.bin";
        let mut parser = Parser::new(path)?;
        assert_eq!(parser.next::<u32>(), 66051);
        assert_eq!(parser.next::<u32>(), 67438087);
        assert_eq!(parser.next::<u32>(), 134810123);
    }

    #[test]
    fn parse_u64() {
        let path = "00_to_ff.bin";
        let mut parser = Parser::new(path)?;
        assert_eq!(parser.next::<u64>(), 283686952306183);
        assert_eq!(parser.next::<u64>(), 579005069656919567);
        assert_eq!(parser.next::<u64>(), 1157726452361532951);
    }

    #[test]
    fn skip() {
        let path = "sample.pcap";
        let mut parser = Parser::new(path)?;
        assert_eq!(parser.get_file_pos(), 0);
        parser.skip(4000)?;
        assert_eq!(parser.get_file_pos(), 4000);
    }

    #[test]
    fn parse_global_pcap_header() {
        let path = "sample.pcap";
        let mut parser = Parser::new(path)?;
        let gpcap = GlobalPcapHeader::parse(&mut parser)?;
        println!("{:?}", gpcap);
    }


    #[test]
    fn parse() {
        crate::parse();
    }
}
