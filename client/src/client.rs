use std::io::{Read, Write};
use std::net::TcpStream;
use bytes::Bytes;
use smq_lib::enums::errors::ClientError;
use smq_lib::structs::message::Message;
use smq_lib::traits::client::Client;

const PUSH_HEADER: [u8; 1] = [0];
const PULL_HEADER: [u8; 1] = [1];

pub struct ClientImpl {
    stream: Option<TcpStream>,
}

impl ClientImpl {
    pub fn new() -> Self {
        ClientImpl {
            stream: None,
        }
    }

    fn get_stream(&mut self) -> Result<&mut TcpStream, ClientError> {
        match self.stream {
            Some(_) => Ok(self.stream.as_mut().unwrap()),
            None => Err(ClientError::StreamNotStarted),
        }
    }
}

impl Client for ClientImpl {
    fn connect(&mut self, host: &str, port: u16) -> Result<(), ClientError> {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr);

        self.stream = match stream {
            Ok(stream) => Some(stream),
            Err(e) => return Err(ClientError::UnableToStartStream(e.to_string())),
        };

        Ok(())
    }

    fn disconnect(&self) {
        todo!()
    }

    fn push(&mut self, message: &Message) -> Result<bool, ClientError> {
        let stream = self.get_stream()?;

        let request = vec![
            Bytes::from(PUSH_HEADER.to_vec()),
            message.serialize(),
        ].concat();
        if let Err(e) = stream.write(&request) {
            return Err(ClientError::CantWriteToStream(e.to_string()));
        };

        let mut response_buf: [u8; 1] = [0];
        if let Err(e) = stream.read(&mut response_buf) {
            return Err(ClientError::CantReadFromStream(e.to_string()))
        };

        Ok(response_buf[0] == 0)
    }

    fn pull(&mut self) -> Result<Message, ClientError> {
        let stream = self.get_stream()?;

        let request = &PULL_HEADER;
        if let Err(e) = stream.write(request) {
            return Err(ClientError::CantWriteToStream(e.to_string()));
        };

        let mut response: Vec::<u8> = vec![];
        if let Err(e) = stream.read_to_end(&mut response) {
            return Err(ClientError::CantReadFromStream(e.to_string()))
        };

        if response[0] != 0 {
            return Err(ClientError::ServerError);
        }

        let response = &response[1..];
        match Message::deserialize(response) {
            Ok(msg) => Ok(msg),
            Err(e) => Err(ClientError::MessageError(e)),
        }
    }
}