use moex_pcap_parser_rust::parse;

fn main() {
    match parse() {
        Ok(_) => ":)",
        Err(_) => ":(",
    };
}
