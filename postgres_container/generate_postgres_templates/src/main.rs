use std::{env, path};
use tokio::fs;

mod config;

use crate::config::Config;

#[tokio::main]
async fn main() {
    let args = match env::args().nth(1) {
        Some(a) => path::PathBuf::from(a),
        None => return println!("argument error:\nconfig file not found."),
    };
    let config = match config::from_filepath(&args).await {
        Ok(c) => c,
        Err(e) => return println!("configuration error:\n{}", e),
    };

    if let Err(e) = generate_data_dir(&config).await {
        return println!("data dir error:\n{}", e);
    };
    if let Err(e) = generate_podmanfile(&config).await {
        return println!("podmanfile error:\n{}", e);
    };
    if let Err(e) = generate_podman_compose(&config).await {
        return println!("podman-compose error:\n{}", e);
    };
}

async fn generate_podmanfile(config: &Config) -> Result<(), String> {
    let podmanfile = match fs::read("../templates/podmanfile").await {
        Ok(contents) => contents,
        Err(e) => return Err(e.to_string()),
    };

    let mut path = config.directory.clone();
    path.push("podmanfile");

    match fs::write(path, podmanfile).await {
        Ok(b) => Ok(b),
        Err(e) => return Err(e.to_string()),
    }
}

async fn generate_podman_compose(config: &Config) -> Result<(), String> {
    let podman_compose = match fs::read_to_string("../templates/podman-compose.yml").await {
        Ok(contents) => contents
            .replace("${port}", &config.port.to_string())
            .replace("${postgres_password}", &config.postgres_password),
        Err(e) => return Err(e.to_string()),
    };

    let mut path = config.directory.clone();
    path.push("podman-compose.yml");

    match fs::write(path, podman_compose.as_bytes()).await {
        Ok(b) => Ok(b),
        Err(e) => return Err(e.to_string()),
    }
}

async fn generate_data_dir(config: &Config) -> Result<(), String> {
    let mut path = config.directory.clone();
    path.push("data");

    match fs::create_dir_all(path).await {
        Ok(b) => Ok(b),
        Err(e) => Err(e.to_string()),
    }
}
