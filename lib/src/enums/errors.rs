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

#[derive(Debug)]
pub enum ClientError {
    UnableToStartStream(String),
    StreamNotStarted,
    CantWriteToStream(String),
    CantReadFromStream(String),
    ServerError,
    MessageError(MessageError),
}
