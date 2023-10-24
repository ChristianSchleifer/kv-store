use kv_store::server::Server;

fn main() {
    let mut server = Server::new("127.0.0.1:7878");
    server.run();
}