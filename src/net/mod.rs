use crate::net::server::Server;

pub mod server;

pub async fn init() -> Server {
    return server::init().await;
}
