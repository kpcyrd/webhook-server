use reduce::Reduce;
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender, channel};

use std::path::PathBuf;
use std::process::{Command, Child};

use ::Result;
use config::Config;


#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub hook: String,
}

impl Message {
    pub fn new<I: Into<String>>(hook: I) -> Message {
        Message {
            hook: hook.into(),
        }
    }
}

pub fn spawn_child(me: &PathBuf, config: &Config) -> Result<(Child, IpcReceiver<Message>)> {
    let (server0, server0_name) = IpcOneShotServer::new()?;

    let addr = config.addr.to_string();
    let hook_list = config.hooks.keys()
        .map(|x| x.to_owned())
        .reduce(|x, y| x + "," + &y)
        .unwrap_or(String::new());

    let child = Command::new(me)
        .args(&["--child", &server0_name])
        .env("NARNIA_HOOKS", hook_list)
        .env("NARNIA_ADDR", addr)
        .spawn()?;

    let (_, rx) = server0.accept().unwrap();
    Ok((child, rx))
}

pub fn connect_parent(path: String) -> Result<IpcSender<Message>> {
    let (tx, rx) = channel()?;
    let origin = IpcSender::connect(path)?;
    origin.send(rx).unwrap();

    Ok(tx)
}
