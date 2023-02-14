#[derive(Debug, PartialEq)]
pub enum MessageError {
    InvalidType,
    InvalidDataLength,
    InvalidData,
    InvalidHeaderBits,
}

pub enum ServerError {
    UnableToStartListener(String),
}