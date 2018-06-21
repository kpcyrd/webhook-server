#![warn(unused_extern_crates)]
extern crate tk_http;
extern crate tk_listen;
extern crate tokio_core;
extern crate futures;
extern crate ipc_channel;
extern crate serde_json;
extern crate reduce;
extern crate toml;
#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;

#[cfg(target_os="linux")]
extern crate syscallz;

mod errors {
    use std;
    use toml;
    use syscallz;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            AddrParse(std::net::AddrParseError);
            Toml(toml::de::Error);
            Seccomp(syscallz::Error) #[cfg(target_os="linux")];
        }
    }
}
pub use errors::Result;

pub mod child;
pub mod config;
pub mod http;
pub mod parent;
pub mod sandbox;
