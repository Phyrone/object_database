use crate::resources::Resources;
use log::{debug, info};
use quiche::{accept, Config, Connection, ConnectionId};
use std::borrow::Borrow;
use std::net::SocketAddr;
use std::pin::Pin;
use std::str::FromStr;
use std::thread::{current, sleep};
use std::time::Duration;

pub async fn init() -> Server {
    let mut config = Config::new(quiche::PROTOCOL_VERSION).expect("Config::new");

    info!("init_quiche_server");
    current()
        .name()
        .map(|name| debug!("init_quiche_server: thread name: {}", name));

    let id = ConnectionId::default();
    let acceptor = accept(
        &id,
        Option::None,
        SocketAddr::from_str("127.0.0.1:4433").expect("could not parse socket address"),
        &mut config,
    )
        .expect("could not start acceptor");

    return Server { config, acceptor };
}

pub struct Server {
    config: Config,
    acceptor: Pin<Box<Connection>>,
}

impl Server {
    pub async fn run(&mut self) {
        loop {
            //TODO server implementation
            sleep(Duration::from_secs(4));
        }
    }
}
