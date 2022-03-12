use crate::Parser;



pub trait Packet {
    fn parse(parser: &mut Parser, size: u64) -> Self;
}

pub trait Order {
    fn parse(&mut self, parser: &mut Parser) -> Self;
}