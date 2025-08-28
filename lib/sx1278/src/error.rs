
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SX1278Error {
    SPIError(i32),
    ParseError,
    SerializationError,
    InvalidDevice(u8, u8),
    ConfigurationError,
    Other
}
