use std::mem;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use crate::errors::CustomErrors;
use crate::glob_pcap_header_parser::Ordering;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Endian {
    Big,
    Little,
}

#[allow(unused_must_use)]
impl Endian {
    pub fn get_ordering(magic: u32) -> Result<Endian, CustomErrors> {
        Ok(match magic {
            0xA1B2C3D4 | 0xA1B23C4D => Endian::Big,
            0xD4C3B2A1 | 0x4D3CB2A1 => Endian::Little,
            _ => return Err(CustomErrors::BadMagicNumberError)
        })
    }
}


#[derive(Debug)]
pub struct Parser {
    buffer: [u8; Parser::BUFFER_MAX_SIZE],
    buffer_pos: usize,
    is_init: bool,
    endian: Endian,
    buffered_reader: BufReader<File>,
}

impl Parser {
    const BUFFER_MAX_SIZE: usize = 4096;
    pub fn new(path: &str) -> Result<Parser, CustomErrors> {
        match File::open(Path::new(path)) {
            Ok(f) => {
                Ok(Parser {
                buffer: [0; Parser::BUFFER_MAX_SIZE],
                buffer_pos: 0,
                is_init: false,
                endian: Endian::Big,
                buffered_reader: BufReader::new(f),
            })
            },
            Err(e) => return Err(CustomErrors::FailedToOpenFile)
        }
    }

    pub fn set_endian(&mut self, endian: &Ordering) {
        self.endian = match endian {
            Ordering::BigEndianNanoseconds(_) | Ordering::BigEndianMilliseconds(_) => Endian::Big,
            Ordering::LittleEndianNanoseconds(_) | Ordering::LittleEndMilliseconds(_) => Endian::Little,
        }
    }

    pub fn next_le<T>(&mut self) -> T
    where T: FromBytes {
        self.next_helper::<T>(Endian::Little)
    }

    pub fn next_be<T>(&mut self) -> T
    where T: FromBytes {
        self.next_helper::<T>(Endian::Big)
    }

    pub fn next<T>(&mut self) -> T
    where T: FromBytes {
        self.next_helper::<T>(self.endian.clone())
    }

    fn init(&mut self) {
        self.buffered_reader.read_exact(&mut self.buffer[0..Parser::BUFFER_MAX_SIZE]);
        self.is_init = true;
    }

    #[allow(unused_must_use)]
    fn next_helper<T>(&mut self, endian: Endian) -> T
        where T: FromBytes {
        let type_size = mem::size_of::<T>();
        if !self.is_init {
            self.init();
        }

        if type_size > Parser::BUFFER_MAX_SIZE - self.buffer_pos {
            self.fill_buffer();     // TODO: process later
            self.buffer_pos = 0;
        }
        let bytes = &mut self.buffer[self.buffer_pos .. self.buffer_pos + type_size];
        if endian == Endian::Big {
            bytes.reverse();
        }
        let value: T = T::from_ne_bytes(bytes);

        self.buffer_pos += type_size;

        value
    }

    fn fill_buffer(&mut self) -> Result<(), std::io::Error> {
        let left = Parser::BUFFER_MAX_SIZE - self.buffer_pos;
        for i in 0..left {
            self.buffer[i] = self.buffer[Parser::BUFFER_MAX_SIZE - left + i];
        }
        self.buffered_reader.read_exact(&mut self.buffer[left..Parser::BUFFER_MAX_SIZE]);
        Ok(())
    }

    pub fn next_mac(&mut self) -> [u8; 6] {
        let mut mac: [u8; 6] = [0, 0, 0, 0, 0, 0];
        for i in 0..6 {
            mac[i] = self.next::<u8>();
        }
        mac
    }

    pub fn next_ip_v4(&mut self) -> [u8; 4] {
        let mut mac: [u8; 4] = [0, 0, 0, 0];
        for i in 0..4 {
            mac[i] = self.next::<u8>();
        }
        mac
    }

    pub fn get_file_pos(&mut self) -> u64 {
        self.buffered_reader.seek(SeekFrom::Current(0)).unwrap()
    }

    pub fn skip(&mut self, n: usize) -> Result<(), std::io::Error>{
        if n == 0 {
            return Ok(())
        } else {
            if Parser::BUFFER_MAX_SIZE > self.buffer_pos + n {
                self.buffer_pos += n;
            } else {
                let left_in_buf = Parser::BUFFER_MAX_SIZE - self.buffer_pos;
                self.buffered_reader.seek_relative((n - left_in_buf) as i64);
                self.buffered_reader.read_exact(&mut self.buffer);
                self.buffer_pos = 0;
            }
        }
        Ok(())
    }
}

pub trait FromBytes: Sized {
    fn from_ne_bytes(bytes: &[u8]) ->Self;
}

impl FromBytes for u8 {
    fn from_ne_bytes(bytes: &[u8]) -> Self {
        bytes.try_into().map(u8::from_ne_bytes).ok().unwrap()
    }
}

impl FromBytes for u16 {
    fn from_ne_bytes(bytes: &[u8]) -> Self {
        bytes.try_into().map(u16::from_ne_bytes).ok().unwrap()
    }
}

impl FromBytes for i16 {
    fn from_ne_bytes(bytes: &[u8]) -> Self {
        bytes.try_into().map(i16::from_ne_bytes).ok().unwrap()
    }
}

impl FromBytes for u32 {
    fn from_ne_bytes(bytes: &[u8]) -> Self {
        bytes.try_into().map(u32::from_ne_bytes).ok().unwrap()
    }
}

impl FromBytes for i32 {
    fn from_ne_bytes(bytes: &[u8]) -> Self {
        bytes.try_into().map(i32::from_ne_bytes).ok().unwrap()
    }
}

impl FromBytes for u64 {
    fn from_ne_bytes(bytes: &[u8]) -> Self {
        bytes.try_into().map(u64::from_ne_bytes).ok().unwrap()
    }
}

impl FromBytes for i64 {
    fn from_ne_bytes(bytes: &[u8]) -> Self {
        bytes.try_into().map(i64::from_ne_bytes).ok().unwrap()
    }
}
