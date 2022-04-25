use std::error::Error;
use crate::bootstrap::BootstrapConfiguration;

pub mod bootstrap;
mod test;
mod query;
mod types;
mod state;


impl Database {
    pub fn new(_bootstrap: &BootstrapConfiguration) -> Self {
        return Self {};
    }

    pub async fn run_loop(&self) {

    }
}


pub struct Database;