#[derive(Debug)]
pub enum CustomErrors {
    FailedToOpenFile,
    BadMagicNumberError,
    BadMessageTypeError,
    UnsupportedProtocolVersion,
    UnsupportedProtocol,
}
