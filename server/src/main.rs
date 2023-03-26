mod server;

use server::ServerImpl;
use smq_lib::traits::server::Server;

fn main() {
    ::std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let mut server = ServerImpl::new();
    server.bind(None).expect("Server encountered an error");

    server
        .r#loop()
        .expect("An error occurred when running server loop");

    server.stop().expect("Can't stop TCP listener");
}
