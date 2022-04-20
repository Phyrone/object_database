use std::io::Error;
use crate::resources::Resources;
use crate::CommandLine;
use config::{AsyncConfigBuilder, Config};
use log::{debug, error};
use rust_embed::EmbeddedFile;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn init(command_line: &CommandLine) -> Config {
    let config_file = command_line.config_file.as_str();
    debug!("config_file={}", config_file);
    if !Path::new(config_file).exists() {
        match save_default_config(config_file).await {
            Ok(_) => debug!("default config file created"),
            Err(e) => error!("failed to create default config file: {}", e),
        };
    }

    let config = Config::builder()
        .add_source(config::File::with_name(config_file).required(false))
        .build()
        .expect("Failed to load config");
    debug!("Config loaded {:?}", config);

    return config;
}

async fn save_default_config(config_file: &str) -> Result<(), Error> {
    let mut file = File::create(config_file).await?;
    let default_config: EmbeddedFile = Resources::get("config.yml").unwrap();
    for chunk in default_config.data.chunks(1024) {
        file.write(chunk).await?;
        file.flush().await.expect("Failed to flush");
    }
    Ok(())
}