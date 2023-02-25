use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use bytes::Bytes;
use log::{error, info};
use smq_lib::enums::errors::{MessageError, ServerError};
use smq_lib::structs::message::Message;
use smq_lib::traits::server::Server;

pub(crate) struct ServerImpl {
    queue: Arc<Mutex<VecDeque<Message>>>,
    threads: Vec<JoinHandle<()>>,
    listener: Option<TcpListener>,
}

impl ServerImpl {
    pub fn new() -> Self {
        ServerImpl {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            threads: vec![],
            listener: None,
        }
    }
}

const SUCCESS_HEADER: [u8; 1] = [0];
const FAILED_HEADER: [u8; 1] = [1];

impl ServerImpl {
    fn handle_incoming(queue: Arc<Mutex<VecDeque<Message>>>, mut stream: TcpStream) {
        info!("Started a TCP handler");
        loop {
            let mut header: [u8; 9] = [0; 9];
            // TODO: handle this expect
            stream.read_exact(&mut header).expect("Can't read headers");
            let first_byte = header[0];
            let size = u64::from_be_bytes([header[1], header[2], header[3], header[4],
                header[5], header[6], header[7], header[8]]);

            if first_byte == 0 {
                let mut body = vec![0_u8; size as usize];
                // TODO: handle this expect
                stream.read_exact(&mut body).expect("Can't read body");
                info!("Got a push message");
                // push
                let msg = match Message::deserialize(&body) {
                    Ok(m) => m,
                    Err(_) => return stream.write_all(&FAILED_HEADER).unwrap(),
                };

                let queue = &mut queue.lock().unwrap();
                match ServerImpl::enqueue(queue, msg) {
                    Ok(_) => stream.write_all(&SUCCESS_HEADER).unwrap(),
                    Err(_) => stream.write_all(&FAILED_HEADER).unwrap(),
                }
            } else {
                info!("Got a pull message");
                // pull
                let queue = &mut queue.lock().unwrap();
                let msg = ServerImpl::dequeue(queue).serialize();
                let response = vec![
                    Bytes::from(SUCCESS_HEADER.to_vec()),
                    Bytes::from(msg.len().to_be_bytes().to_vec()),
                    msg,
                ].concat();
                stream.write_all(&response).unwrap();
            }
        }
    }
}

impl Server for ServerImpl {
    fn bind(&mut self, port: Option<usize>) -> Result<(), ServerError> {
        let addr = format!("0.0.0.0:{}", port.unwrap_or(8080));

        info!("Starting TCP listener");
        self.listener = match TcpListener::bind(addr) {
            Ok(listener) => Some(listener),
            Err(e) => return Err(ServerError::UnableToStartServer(e.to_string())),
        };

        info!("Listener is ready to listen to incoming messages");

        Ok(())
    }

    fn r#loop(&mut self) -> Result<(), ServerError> {
        let listener = match &self.listener {
            Some(listener) => listener,
            None => return Err(ServerError::ServerNotYetStarted),
        };

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    error!("Can't decode stream, error: {}", e);
                    continue;
                }
            };

            let queue = self.queue.clone();
            let t = thread::spawn(move || {
                ServerImpl::handle_incoming(queue, stream)
            });
            self.threads.push(t);
        }

        self.stop()
    }

    fn stop(&mut self) -> Result<(), ServerError> {
        let threads = &mut self.threads;
        for thread in threads.drain(..) {
            thread.join().unwrap();
        }

        Ok(())
    }

    fn enqueue(queue: &mut VecDeque<Message>, message: Message) -> Result<(), MessageError> {
        message.validate()?;

        queue.push_back(message);

        Ok(())
    }

    fn dequeue(queue: &mut VecDeque<Message>) -> Message {
        if queue.is_empty() {
            return Message::empty_message();
        }
        queue.pop_front().unwrap()
    }
}