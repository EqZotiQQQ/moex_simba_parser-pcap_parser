#[derive(Debug)]
#[allow(unused_must_use)]
pub enum CustomErrors {
    BadMagicNumberError,
    BadMessageTypeError,
    UnsupportedProtocolVersion,
    UnsupportedProtocol,
}
