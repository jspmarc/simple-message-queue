use smq_lib::structs::message::Message;
use smq_lib::traits::client::Client;
use crate::client::ClientImpl;

mod client;

fn main() {
    ::std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let mut client = ClientImpl::new();

    client.connect("localhost", 8080).expect("Can't connect to server");

    let msg = Message::from_u16_arr(&[1,2]);
    let result = client.push(&msg).expect("Can't push message");
    if !result {
        panic!("Failed to push message, server can't receive");
    }

    let result = client.pull().expect("Can't pull message");
    println!("{:#?}", result);
}
