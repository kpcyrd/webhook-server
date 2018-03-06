use http;
use sandbox;
use sandbox::ipc::Message;

use std::env;
use std::net::SocketAddr;
use std::collections::HashSet;

use errors::ResultExt;
use ::Result;

#[derive(Debug)]
struct SandboxConfig {
    addr: SocketAddr,
    hooks: HashSet<String>,
}

impl SandboxConfig {
    #[inline]
    fn from_env() -> Result<SandboxConfig> {
        let addr = env::var("NARNIA_ADDR")
            .chain_err(|| "listen addr not set in child")?;
        let addr = addr.parse()?;

        let list = env::var("NARNIA_HOOKS")
            .chain_err(|| "webhook list not set in child")?;
        let hooks = list.split(",")
            .map(|x| x.to_owned())
            .collect();

        Ok(SandboxConfig {
            addr,
            hooks,
        })
    }
}

pub fn run(socket: String) -> Result<()> {
    info!("ipc path is {:?}", socket);

    let tx = sandbox::ipc::connect_parent(socket)?;

    let config = SandboxConfig::from_env()?;

    info!("booting server");
    let server = http::Server::new(config.addr, config.hooks, tx.clone());
    tx.send(Message::new("is up")).unwrap();
    server.start()?;

    Ok(())
}
