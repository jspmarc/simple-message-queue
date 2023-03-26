use crate::client::ClientImpl;
use log::info;
use smq_lib::structs::message::Message;
use smq_lib::traits::client::Client;
use std::thread;
use std::time::Duration;

mod client;

fn main() {
    ::std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let mut threads: Vec<thread::JoinHandle<()>> = vec![];
    for id in 0..2000 {
        let t = thread::spawn(move || {
            let mut client = ClientImpl::new();

            client
                .connect("localhost", 8080)
                .expect("Can't connect to server");

            if id % 2 == 0 {
                let msg = Message::from_i32_arr(&[id]);
                let result = client.push(&msg).expect("Can't push message");
                if !result {
                    panic!("Failed to push message, server can't receive");
                }
                info!("client id {} has pushed a message", id);
            } else {
                let result = client.pull().expect("Can't pull message");
                println!("client id {} pulled:\n{:#?}", id, result);
            }

            thread::sleep(Duration::from_millis(100));
            client.disconnect().expect("Can't disconnect");
        });
        threads.push(t);
    }

    for t in threads {
        let _ = t.join();
    }
}
