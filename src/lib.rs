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
extern crate libc;
#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;

#[cfg(target_os="linux")]
extern crate seccomp_sys;

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

pub mod child;
pub mod config;
pub mod http;
pub mod parent;
pub mod sandbox;
