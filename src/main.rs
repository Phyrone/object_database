extern crate core;

mod cli;
mod config;
mod data;
mod logging;
mod net;
mod resources;
mod utils;

use std::borrow::BorrowMut;
use ::config::Config;
use clap;
use clap::{App, Parser};
use log4rs::Handle;
use cli::cli_spec::CommandLine;
use log::debug;
use log::info;
use crate::net::server::Server;

#[tokio::main]
async fn main() {
    let command_line = CommandLine::parse();
    let logger_handle = logging::init(&command_line);
    let config = config::init(&command_line);
    let server = net::init();
    let log_future = utils::log_const();
    debug!("parsed {:?}", command_line);
    info!("Starting ObjectDatabase... version={}", utils::VERSION);
    let server = server.await;
    let config = config.await;

    let mut app = ApplicationStack {
        command_line,
        logger: logger_handle,
        server,
        config,
    }.start();
    log_future.await;
    app.await;
}

pub struct ApplicationStack {
    pub command_line: CommandLine,
    pub logger: Handle,
    pub server: Server,
    pub config: Config,
}

impl ApplicationStack {
    pub async fn start(mut self) {
        info!("Starting server...");
        let future = self.server.borrow_mut().run();
        info!("Server started");
        future.await;
    }
}
