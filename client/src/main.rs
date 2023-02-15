use std::str::FromStr;
use smq_lib::structs::message::Message;
use smq_lib::traits::client::Client;
use crate::client::ClientImpl;

mod client;

fn main() {
    let mut client = ClientImpl::new();

    client.connect("localhost", 8080).expect("Can't connect to server");

    let msg = Message::from_str("123").expect("Can't create message");
    let result = client.push(&msg).expect("Can't push message");
    if !result {
        panic!("Failed to push message, server can't receive");
    }

    let result = client.pull().expect("Can't pull message");
    println!("{:?}", result);
}
