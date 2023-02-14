use crate::enums::errors::{MessageError, ServerError};
use crate::structs::message::Message;

pub trait Server {
    /// a method to start the server
    fn start(&mut self, port: Option<usize>) -> Result<(), ServerError>;

    /// a method to stop the server
    fn stop(&self) -> Result<(), ServerError>;

    /// a method to enqueue a message to the server (usually from outside of the
    /// client)
    fn enqueue(&mut self, message: Message) -> Result<(), MessageError>;

    /// a method to dequeue a message to the server
    fn dequeue(&mut self) -> Message;
}
