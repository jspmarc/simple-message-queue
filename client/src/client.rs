use std::io::{BufReader, Read, Write};
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

        let msg = message.serialize();
        let request = vec![
            Bytes::from(PUSH_HEADER.to_vec()),
            Bytes::from(msg.len().to_be_bytes().to_vec()),
            msg,
        ].concat();
        if let Err(e) = stream.write(&request) {
            return Err(ClientError::CantWriteToStream(e.to_string()));
        };

        let mut buf_reader = BufReader::new(stream);
        let mut response_buf: [u8; 1] = [0];
        if let Err(e) = buf_reader.read_exact(&mut response_buf) {
            return Err(ClientError::CantReadFromStream(e.to_string()));
        };

        Ok(response_buf[0] == 0)
    }

    fn pull(&mut self) -> Result<Message, ClientError> {
        let stream = self.get_stream()?;

        let header = [PULL_HEADER.to_vec(), vec![0_u8; 8]].concat();
        if let Err(e) = stream.write(&header) {
            return Err(ClientError::CantWriteToStream(e.to_string()));
        };

        let mut buf_reader = BufReader::new(stream);
        let mut header: [u8; 9] = [0; 9];
        if let Err(e) = buf_reader.read_exact(&mut header) {
            return Err(ClientError::CantReadFromStream(e.to_string()));
        }

        if header[0] != 0 {
            return Err(ClientError::ServerError(String::from("Server can't send data")));
        }

        let size = u64::from_be_bytes([header[1], header[2], header[3], header[4],
            header[5], header[6], header[7], header[8]]);

        let mut response = vec![0_u8; size as usize];
        if let Err(e) = buf_reader.read_exact(&mut response) {
            return Err(ClientError::CantReadFromStream(e.to_string()));
        }

        match Message::deserialize(&response) {
            Ok(msg) => Ok(msg),
            Err(e) => Err(ClientError::MessageError(e)),
        }
    }
}