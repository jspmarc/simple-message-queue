use crate::enums::r#type::Type;
use crate::structs::message::Message;

trait Client {
    /// a method to connect the client to the server
    fn connect(&self, host: &str, port: u16);

    /// a method to disconnect the client from the server
    fn disconnect(&self);

    /// pushes message to the server's queue
    fn push(&self, message: &Message) -> Type;

    /// pulls a message from the server's queue
    fn pull(&self) -> Message;
}
