use std::collections::VecDeque;
use smq_lib::enums::errors::MessageError;
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

impl Server for ServerImpl {
    fn start(&self, port: Option<usize>) {
        todo!()
    }

    fn stop(&self) {
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