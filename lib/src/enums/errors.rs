#[derive(Debug, PartialEq)]
pub enum MessageError {
    InvalidType,
    InvalidDataLength,
    InvalidData,
    InvalidHeaderBits,
}

#[derive(Debug)]
pub enum ServerError {
    UnableToStartListener(String),
}