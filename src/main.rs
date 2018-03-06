extern crate tk_http;
extern crate tk_listen;
extern crate tokio_core;
extern crate env_logger;
extern crate futures;
extern crate ipc_channel;
extern crate clap;
extern crate serde_json;
extern crate reduce;
extern crate toml;
#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;

mod args;
mod child;
mod config;
mod http;
mod parent;
mod sandbox;

use std::env;

use error_chain::ChainedError;

mod errors {
    use std;
    use toml;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            AddrParse(std::net::AddrParseError);
            Toml(toml::de::Error);
        }
    }
}
pub use errors::Result;

#[inline]
fn run() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let matches = args::get_matches();

    // TODO: maybe also assert sandbox env var is set
    if let Some(socket) = matches.value_of("child") {
        child::run(socket.to_owned())
    } else {
        let config = config::parse_from_file("narnia.toml")?; // TODO
        parent::run(config)
    }
}

fn main() {
    if let Err(ref e) = run() {
        eprintln!("{}", e.display_chain());
        std::process::exit(1);
    }
}
