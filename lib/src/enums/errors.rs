#[derive(Debug, PartialEq)]
pub enum MessageError {
    InvalidType,
    InvalidDataLength,
    InvalidData,
    InvalidHeaderBits,
}