This is MOEX Simba PCAP parser that was written using Rust

src folder contains parsers for ip, udp, pcap headers

src/moex contains MOEX specific parsers

src/utils contains time conversations

Pcap can be passed via simple cli like this:
cargo run "my/great/path/pcap" "42"

At this moment console out available only.

Sample PCAP file http://ftp.moex.ru/pub/SIMBA/Spectra/prod/pcap/2021-11-10.1844-1910.zip
