#![feature(nll, extern_prelude, pin)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate actix;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;
extern crate notify_rust;
#[macro_use]
extern crate lazy_static;
extern crate directories;
#[macro_use]
extern crate structopt;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate alsa;
extern crate chrono;
extern crate inotify;
extern crate ron;

mod config;
mod formatter;
mod output;
mod parse;
mod statusbar;
mod util;
mod widget;

use config::Config;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Write default config
    #[structopt(long = "write-default")]
    write_default: bool,
}

fn run() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    env_logger::init();

    let cfg = if opt.write_default {
        Config::write_default()?
    } else {
        Config::load_or_write_default()?
    };

    //statusbar::run(cfg);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
