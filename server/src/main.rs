mod server;

use server::ServerImpl;
use smq_lib::traits::server::Server;

fn main() {
    ::std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let mut server = ServerImpl::new();
    server.start(None).expect("Server encountered an error");
}
