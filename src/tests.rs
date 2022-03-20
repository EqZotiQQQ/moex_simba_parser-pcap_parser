#[cfg(test)]
mod tests {
    use crate::glob_pcap_header::GlobalPcapHeader;
    use crate::parser::Parser;

    #[test]
    fn parse_u8() {
        let path = "samples/00_to_ff.bin";
        let mut parser = Parser::new(path).unwrap();
        assert_eq!(parser.next::<u8>().unwrap(), 0);
        assert_eq!(parser.next::<u8>().unwrap(), 1);
        assert_eq!(parser.next::<u8>().unwrap(), 2);
    }

    #[test]
    fn parse_u16() {
        let path = "samples/00_to_ff.bin";
        let mut parser = Parser::new(path).unwrap();
        assert_eq!(parser.next::<u16>().unwrap(), 1);
        assert_eq!(parser.next::<u16>().unwrap(), 515);
        assert_eq!(parser.next::<u16>().unwrap(), 1029);
    }

    #[test]
    fn parse_u32() {
        let path = "samples/00_to_ff.bin";
        let mut parser = Parser::new(path).unwrap();
        assert_eq!(parser.next::<u32>().unwrap(), 66051);
        assert_eq!(parser.next::<u32>().unwrap(), 67438087);
        assert_eq!(parser.next::<u32>().unwrap(), 134810123);
    }

    #[test]
    fn parse_u64() {
        let path = "samples/00_to_ff.bin";
        let mut parser = Parser::new(path).unwrap();
        assert_eq!(parser.next::<u64>().unwrap(), 283686952306183);
        assert_eq!(parser.next::<u64>().unwrap(), 579005069656919567);
        assert_eq!(parser.next::<u64>().unwrap(), 1157726452361532951);
    }

    #[test]
    fn skip() {
        let path = "sample.pcap";
        let mut parser = Parser::new(path).unwrap();
        parser.skip(4000).unwrap();
    }

    #[test]
    fn parse_global_pcap_header() {
        let path = "sample.pcap";
        let mut parser = Parser::new(path).unwrap();
        let gpcap = GlobalPcapHeader::parse(&mut parser).unwrap();
        println!("{:?}", gpcap);
    }


    #[test]
    fn parse() {
        crate::parse("sample.pcap", 42).unwrap();
    }
}
