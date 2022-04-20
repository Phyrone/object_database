use log4rs::append::console::ConsoleAppender;
use log4rs::{Config, Handle};
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;
use crate::CommandLine;

const CONSOLE_APPENDER_NAME: &str = "console";

pub fn init(command_line: &CommandLine) -> Handle {
    let config = create_logger_config(command_line);
    let log4rs = log4rs::init_config(config);
    match log4rs {
        Err(e) => panic!("Logger initialization failed with error: {}", e),
        Ok(result) => { return result; }
    }
}

fn create_logger_config(command_line: &CommandLine) -> Config {
    let config = Config::builder()
        .appender(create_console_appender())
        .build(create_root_logger(&command_line.log_level));
    match config {
        Err(errors) => panic!("Logger configuration failed"),
        Ok(result) => { return result; }
    }
}

fn create_root_logger(level: &LevelFilter) -> Root {
    let root = Root::builder()
        .appender(CONSOLE_APPENDER_NAME)
        .build(level.clone());
    return root;
}

fn create_console_appender() -> Appender {
    let appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h({l} {d(%S:%M:%S)} {t} - {m})}{n}")))
        .build();
    return Appender::builder()
        .build(CONSOLE_APPENDER_NAME, Box::new(appender));
}