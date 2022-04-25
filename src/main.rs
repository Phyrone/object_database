extern crate core;

mod cli;
mod config;
mod data;
mod logging;
mod resources;
mod test;
mod utils;

use ::config::Config;
use clap;
use clap::Parser;
use log4rs::Handle;
use cli::cli_spec::CommandLine;
use log::debug;
use log::info;
use utils::log_const;
use crate::core::bootstrap::BootstrapConfiguration;
use crate::core::Database;

#[tokio::main]
async fn main() {
    let command_line = CommandLine::parse();
    let logger_handle = logging::init(&command_line);
    let config = config::init(&command_line);

    log_const();
    debug!("parsed {:?}", command_line);
    info!("Starting ObjectDatabase... version={}", utils::VERSION);
    let config = config.await;
    let bootstrap_config = BootstrapConfiguration::new();
    let database = Database::new(&bootstrap_config);
    ApplicationStack {
        command_line,
        logger: logger_handle,
        config,
        database,
    }.start().await;
}

pub struct ApplicationStack {
    pub command_line: CommandLine,
    pub logger: Handle,
    pub config: Config,
    pub database: Database,
}

impl ApplicationStack {
    pub async fn start(self) {
        let database_loop = self.database.run_loop();

        database_loop.await;
    }
}
