use clap::Parser;
use log::LevelFilter;

#[derive(Parser, Debug)]
#[clap(author="Phyrone", version, about, long_about = None)]
pub struct CommandLine {

    #[clap(short='l', long="level",default_value = "info")]
   pub log_level: LevelFilter,

    #[clap(short='c', long="config",default_value = "config.yml")]
    pub config_file: String,

}