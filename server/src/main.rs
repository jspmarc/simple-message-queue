mod server;

use server::ServerImpl;
use smq_lib::traits::server::Server;

fn main() {
    let server = ServerImpl::new();
    server.start(None);
}
