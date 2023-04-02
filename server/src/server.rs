use bytes::Bytes;
use log::{error, info};
use smq_lib::enums::errors::{MessageError, ServerError};
use smq_lib::structs::message::Message;
use smq_lib::traits::server::Server;
use std::collections::{HashMap, VecDeque};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use uuid::Uuid;

pub(crate) struct ServerImpl {
    queue: Arc<RwLock<VecDeque<Message>>>,
    threads: Arc<Mutex<HashMap<Uuid, JoinHandle<()>>>>,
    listener: Option<TcpListener>,
}

impl ServerImpl {
    pub fn new() -> Self {
        ServerImpl {
            queue: Arc::new(RwLock::new(VecDeque::new())),
            threads: Arc::new(Mutex::new(HashMap::new())),
            listener: None,
        }
    }
}

const SUCCESS_HEADER: [u8; 1] = [0];
const FAILED_HEADER: [u8; 1] = [1];

impl ServerImpl {
    fn handle_incoming(
        queue: Arc<RwLock<VecDeque<Message>>>,
        mut stream: TcpStream,
        id: Uuid,
        tx: mpsc::Sender<Uuid>,
    ) {
        info!("Started a TCP handler");
        loop {
            let mut header: [u8; 9] = [0; 9];
            // TODO: handle this expect
            stream.read_exact(&mut header).expect("Can't read headers");
            let first_byte = header[0];
            let size = u64::from_be_bytes([
                header[1], header[2], header[3], header[4], header[5], header[6], header[7],
                header[8],
            ]);

            let response = {
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

                    let queue = &mut queue.write().unwrap();
                    let response = match ServerImpl::enqueue(queue, msg) {
                        Ok(_) => SUCCESS_HEADER,
                        Err(_) => FAILED_HEADER,
                    };
                    [response.to_vec(), vec![0; 8]].concat()
                } else if first_byte == 1 {
                    info!("Got a pull message");
                    // pull
                    let queue = &mut queue.write().unwrap();
                    let msg = ServerImpl::dequeue(queue).serialize();
                    vec![
                        Bytes::from(SUCCESS_HEADER.to_vec()),
                        Bytes::from(msg.len().to_be_bytes().to_vec()),
                        msg,
                    ]
                    .concat()
                } else {
                    let _ = tx.send(id);
                    break;
                }
            };
            stream
                .write_all(&response)
                .expect("Failed to send response");
        }
    }
}

impl Server for ServerImpl {
    fn bind(&mut self, port: Option<usize>) -> Result<(), ServerError> {
        let addr = format!("0.0.0.0:{}", port.unwrap_or(8080));

        info!("Starting TCP listener");
        let listener = match TcpListener::bind(addr) {
            Ok(listener) => listener,
            Err(e) => return Err(ServerError::UnableToStartServer(e.to_string())),
        };
        let _ = listener.set_nonblocking(true);
        self.listener = Some(listener);

        info!("Listener is ready to listen to incoming messages");

        Ok(())
    }

    fn r#loop(&mut self) -> Result<(), ServerError> {
        let listener = match &self.listener {
            Some(listener) => listener,
            None => return Err(ServerError::ServerNotYetStarted),
        };

        let (tx_id, rx_id) = mpsc::channel::<Uuid>();
        let (tx_done, rx_done) = mpsc::channel::<()>();

        let threads = self.threads.clone();
        let rx_thread = thread::spawn(move || {
            while let Ok(id) = rx_id.recv() {
                let t = threads.lock().unwrap().remove(&id);
                if let Some(t) = t {
                    let _ = t.join();
                }
            }
        });

        let _ = ctrlc::set_handler(move || {
            let _ = tx_done.send(());
            info!("Gracefully shutting down...");
        });

        for stream in listener.incoming() {
            // check if we are done
            match rx_done.try_recv() {
                Err(mpsc::TryRecvError::Disconnected) | Ok(()) => break,
                Err(mpsc::TryRecvError::Empty) => (),
            };

            let stream = match stream {
                Ok(s) => s,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(15));
                    continue;
                }
                Err(e) => {
                    error!("Can't decode stream, error: {}", e);
                    continue;
                }
            };

            let id = Uuid::new_v4();

            let queue = self.queue.clone();
            let tx = tx_id.clone();
            let t = thread::spawn(move || ServerImpl::handle_incoming(queue, stream, id, tx));
            self.threads.lock().unwrap().insert(id, t);
        }

        drop(tx_id);
        let _ = rx_thread.join();

        Ok(())
    }

    fn stop(&mut self) -> Result<(), ServerError> {
        info!("Joining worker threads...");
        let mut threads = self.threads.lock().unwrap();
        for thread in threads.drain() {
            let _ = thread.1.join();
        }

        info!("Good bye~");
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
