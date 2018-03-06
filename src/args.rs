use clap::{self, App, Arg};

pub fn get_matches<'a>() -> clap::ArgMatches<'a> {
    App::new("narnia")
        .arg(Arg::with_name("child")
            .long("child")
            .takes_value(true)
        )
        .get_matches()
}
