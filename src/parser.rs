use std::mem;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use crate::errors::CustomErrors;
use crate::errors::CustomErrors::BadMagicNumberError;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Endian {
    Big,
    Little,
}

impl Endian {
    pub fn get_ordering(magic: u32) -> Result<Endian, CustomErrors> {
        Ok(match magic {
            0xA1B2C3D4 | 0xA1B23C4D => Endian::Big,
            0xD4C3B2A1 | 0x4D3CB2A1 => Endian::Little,
            _ => return Err(BadMagicNumberError)
        })
    }
}

const BUFFER_MAX_SIZE: usize = 2048;

#[derive(Debug)]
pub struct Parser {
    buffer: [u8; BUFFER_MAX_SIZE],
    buffer_pos: usize,
    parsed_bytes: usize,
    endian: Endian,
    buffered_reader: BufReader<File>,
}

impl Parser {
    pub fn new(path: &str) -> Result<Parser, std::io::Error> {
        Ok(Parser {
            buffer: [0; BUFFER_MAX_SIZE],
            buffer_pos: 0,
            parsed_bytes: 0,
            endian: Endian::Big,
            buffered_reader: BufReader::new(File::open(Path::new(path))?),
        })
    }

    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian;
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

    fn next_helper<T>(&mut self, endian: Endian) -> T
        where T: FromBytes {
        let type_size = mem::size_of::<T>();
        if type_size > self.parsed_bytes - self.buffer_pos {
            self.fill_buffer();     // TODO: process later
            self.buffer_pos = 0;
        }

        let bytes = &mut self.buffer[self.buffer_pos .. self.buffer_pos + type_size];
        if endian == Endian::Big {
            bytes.reverse();
        }
        let value: T = T::from_ne_bytes(bytes).unwrap();

        self.buffer_pos += type_size;

        value
    }

    fn fill_buffer(&mut self) -> Result<(), std::io::Error> {
        let left = self.parsed_bytes - self.buffer_pos;
        for i in 0..left {
            self.buffer[i] = self.buffer[BUFFER_MAX_SIZE - left + i];
        }
        self.parsed_bytes = self.buffered_reader.read(&mut self.buffer[left .. BUFFER_MAX_SIZE])? + left;
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
            if BUFFER_MAX_SIZE > self.buffer_pos + n {
                self.buffer_pos += n;
            } else {
                let left_in_buf = BUFFER_MAX_SIZE - self.buffer_pos;
                self.buffered_reader.seek_relative((n - left_in_buf) as i64);
                self.parsed_bytes = self.buffered_reader.read(&mut self.buffer)?;
                self.buffer_pos = 0;
            }
        }
        Ok(())
    }
}

pub trait FromBytes: Sized {
    fn from_ne_bytes(bytes: &[u8]) ->Option<Self>;
}

impl FromBytes for u8 {
    fn from_ne_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.try_into().map(u8::from_ne_bytes).ok()
    }
}

impl FromBytes for u16 {
    fn from_ne_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.try_into().map(u16::from_ne_bytes).ok()
    }
}

impl FromBytes for i16 {
    fn from_ne_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.try_into().map(i16::from_ne_bytes).ok()
    }
}

impl FromBytes for u32 {
    fn from_ne_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.try_into().map(u32::from_ne_bytes).ok()
    }
}

impl FromBytes for i32 {
    fn from_ne_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.try_into().map(i32::from_ne_bytes).ok()
    }
}

impl FromBytes for u64 {
    fn from_ne_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.try_into().map(u64::from_ne_bytes).ok()
    }
}

impl FromBytes for i64 {
    fn from_ne_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.try_into().map(i64::from_ne_bytes).ok()
    }
}
