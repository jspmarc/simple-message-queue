use crate::client::ClientImpl;
use smq_lib::structs::message::Message;
use smq_lib::traits::client::Client;
use std::thread::sleep;
use std::time::Duration;

mod client;

fn main() {
    ::std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let mut client = ClientImpl::new();

    client
        .connect("localhost", 8080)
        .expect("Can't connect to server");

    let msg = Message::from_u16_arr(&[1, 2]);
    let result = client.push(&msg).expect("Can't push message");
    if !result {
        panic!("Failed to push message, server can't receive");
    }

    sleep(Duration::from_secs(1));

    let result = client.pull().expect("Can't pull message");
    println!("{:#?}", result);

    client.disconnect().expect("Can't disconnect");
}
