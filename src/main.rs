use std::env;
use moex_pcap_parser_rust::parse;
use std::path::Path;

fn usage() {
    println!("Usage:");
    println!("cargo run PCAP_FILE_PATH PARSE_N_PACKETS");
    println!("where");
    println!("PCAP_FILE_PATH - String");
    println!("PARSE_N_PACKETS - u64");
}

fn main() {


    let args: Vec<String> = env::args().collect();

    println!("Args = {:?}", args);

    if Path::new(&args[1]).exists() {
        match args[2].parse() {
            Ok(bounds) => {
                match parse(&args[1], bounds) {
                    Ok(_) => ":)",
                    Err(_) => ":(",
                };
            },
            Err(_) => {
                println!("Failed to parse bounds");
                usage();
            }
        };

    } else {
        eprintln!("No such file: {}", args[1]);
    }


}
