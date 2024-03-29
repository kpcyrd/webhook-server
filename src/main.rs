extern crate narnia;
extern crate env_logger;
extern crate clap;
extern crate error_chain;
#[cfg(target_os="linux")]
extern crate seccomp_sys;

mod args;

use std::env;
use error_chain::ChainedError;
use narnia::{child, config, parent, sandbox, Result};


#[inline]
fn run() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let matches = args::get_matches();

    // TODO: maybe also assert sandbox env var is set
    if let Some(socket) = matches.value_of("child") {
        sandbox::activate_stage1().expect("failed to activate stage1");
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
