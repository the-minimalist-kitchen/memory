use serde::{Deserialize, Serialize};
use serde_json;
use std::path;
use tokio::fs;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub port: usize,
    pub directory: path::PathBuf,
    pub memory_policy: String,
    pub maxmemory: String,
    pub maxmemory_samples: usize,
}

pub async fn from_filepath(filepath: &path::PathBuf) -> Result<Config, String> {
    // get position relative to working directory
    let config_pathbuff = match filepath.canonicalize() {
        Ok(pb) => pb,
        Err(e) => return Err(e.to_string()),
    };
    let parent_dir = match config_pathbuff.parent() {
        Some(p) => p,
        _ => return Err("parent directory of config file does not exist.".to_string()),
    };

    let json_str = match fs::read_to_string(&config_pathbuff).await {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    let mut config: Config = match serde_json::from_str(&json_str) {
        Ok(j) => j,
        Err(e) => return Err(e.to_string()),
    };

    // update the directory relative to config filepath
    let directory = match parent_dir.join(&config.directory).canonicalize() {
        Ok(j) => j,
        Err(e) => return Err(e.to_string()),
    };
    if !directory.is_dir() {
        return Err("config.directory is not a directory".to_string());
    }

    config.directory = directory;

    Ok(config)
}
