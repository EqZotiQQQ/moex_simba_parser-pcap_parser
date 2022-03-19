use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;

#[derive(Debug)]
#[allow(unused_must_use)]
pub enum CustomErrors {
    FailedToOpenFile,
    BadMagicNumberError,
    BadMessageTypeError,
    UnsupportedProtocolVersion,
    UnsupportedProtocol,
    UnsupportedDifferentialServiceCodePoint,
    BadMDUpdateActionType,
    BadMDEntryType,
}

impl Display for CustomErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            CustomErrors::FailedToOpenFile => writeln!(f, "Failed to open file"),
            CustomErrors::BadMagicNumberError => writeln!(f, "Bad magic number error"),
            CustomErrors::BadMessageTypeError => writeln!(f, "Bad message type error"),
            CustomErrors::UnsupportedProtocolVersion => writeln!(f, "Unsupported protocol version"),
            CustomErrors::UnsupportedProtocol => writeln!(f, "Unsupported protocol version"),
            CustomErrors::UnsupportedDifferentialServiceCodePoint => writeln!(f, "Unsupported differential service code point"),
            CustomErrors::BadMDUpdateActionType => writeln!(f, "Bad MDUpdate action type"),
            CustomErrors::BadMDEntryType => writeln!(f, "Bad entry type"),
        }
    }
}

impl Error for CustomErrors {

}
