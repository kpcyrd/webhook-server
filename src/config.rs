use toml;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::net::SocketAddr;

use ::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Webhook {
    pub prog: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub cwd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub addr: SocketAddr,
    #[serde(rename="hook")]
    pub hooks: HashMap<String, Webhook>,
}

pub fn parse_from_file(path: &str) -> Result<Config> {
    let mut buf = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut buf)?;

    let config = toml::from_str(&buf)?;
    Ok(config)
}
