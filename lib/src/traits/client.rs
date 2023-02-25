use crate::enums::errors::ClientError;
use crate::structs::message::Message;

pub trait Client {
    /// a method to connect the client to the server
    fn connect(&mut self, host: &str, port: u16) -> Result<(), ClientError>;

    /// a method to disconnect the client from the server
    fn disconnect(&mut self) -> Result<(), ClientError>;

    /// pushes message to the server's queue
    fn push(&mut self, message: &Message) -> Result<bool, ClientError>;

    /// pulls a message from the server's queue
    fn pull(&mut self) -> Result<Message, ClientError>;
}
