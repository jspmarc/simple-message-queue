use std::collections::VecDeque;
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use bytes::Bytes;
use log::{error, info};
use smq_lib::enums::errors::{MessageError, ServerError};
use smq_lib::structs::message::Message;
use smq_lib::traits::server::Server;

pub(crate) struct ServerImpl {
    queue: VecDeque<Message>,
}

impl ServerImpl {
    pub fn new() -> Self {
        ServerImpl {
            queue: VecDeque::new(),
        }
    }
}

const SUCCESS_HEADER: [u8; 1] = [0];
const FAILED_HEADER: [u8; 1] = [1];

impl ServerImpl {
    fn handle_incoming(&mut self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let buffer = buf_reader.buffer();
        let first_byte = buffer[0];
        let body = &buffer[1..];

        if first_byte == 0 {
            info!("Got a push message");
            // push
            let msg = match Message::deserialize(body) {
                Ok(m) => m,
                Err(_) => return stream.write_all(&FAILED_HEADER).unwrap(),
            };

            match self.enqueue(msg) {
                Ok(_) => (),
                Err(_) => stream.write_all(&FAILED_HEADER).unwrap(),
            }
        } else {
            info!("Got a pull message");
            // pull
            let msg = self.dequeue();
            let response = vec![
                Bytes::from(SUCCESS_HEADER.to_vec()),
                msg.serialize()
            ].concat();
            stream.write_all(&response).unwrap();
        }
    }
}

impl Server for ServerImpl {
    fn start(&mut self, port: Option<usize>) -> Result<(), ServerError> {
        let addr = format!("0.0.0.0:{}", port.unwrap_or(8080));

        info!("Starting TCP listener");
        let listener = match TcpListener::bind(addr) {
            Ok(listener) => listener,
            Err(e) => return Err(ServerError::UnableToStartListener(e.to_string())),
        };

        info!("Listener is ready to listen to incoming messages");

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    error!("Can't decode stream, error: {}", e);
                    continue;
                }
            };

            self.handle_incoming(stream);
        }

        Ok(())
    }

    fn stop(&self) -> Result<(), ServerError> {
        todo!()
    }

    fn enqueue(&mut self, message: Message) -> Result<(), MessageError> {
        message.validate()?;

        self.queue.push_back(message);

        Ok(())
    }

    fn dequeue(&mut self) -> Message {
        if self.queue.is_empty() {
            return Message::empty_message();
        }
        self.queue.pop_front().unwrap()
    }
}