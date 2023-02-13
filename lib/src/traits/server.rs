use crate::structs::message::Message;

trait Server {
    /// a method to start the server
    fn start(&self, port: Option<usize>);

    /// a method to stop the server
    fn stop(&self);

    /// a method to dequeue a value from the queue and then broadcast the
    /// message to all connected clients
    fn broadcast(&self);

    /// a method to enqueue a message to the server (usually from outside of the
    /// client)
    fn enqueue(&self, message: Message);

    /// a method to dequeue a message to the server
    fn dequeue(&self) -> Message;
}
