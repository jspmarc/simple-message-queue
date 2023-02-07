use std::any::Any;
use std::rc::Rc;

trait Server {
    /// a method to start the server
    fn start(&self);

    /// pushes message to the server's queue
    fn push(&self, message: Rc<dyn Any>);

    /// pulls a message from the server's queue
    fn pull(&self) -> Rc<dyn Any>;
}
