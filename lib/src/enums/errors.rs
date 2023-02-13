#[derive(Debug)]
pub enum MessageError {
    InvalidType,
    InvalidDataLength,
    InvalidBits,
}