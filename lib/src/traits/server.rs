use crate::enums::errors::{MessageError, ServerError};
use crate::structs::message::Message;
use std::collections::VecDeque;

pub trait Server {
    /// binds the server to a port
    fn bind(&mut self, port: Option<usize>) -> Result<(), ServerError>;

    /// a method to start the server
    fn r#loop(&mut self) -> Result<(), ServerError>;

    /// a method to stop the server
    fn stop(&mut self) -> Result<(), ServerError>;

    /// a method to enqueue a message to the server (usually from outside of the
    /// client)
    fn enqueue(queue: &mut VecDeque<Message>, message: Message) -> Result<(), MessageError>;

    /// a method to dequeue a message to the server
    fn dequeue(queue: &mut VecDeque<Message>) -> Message;
}
