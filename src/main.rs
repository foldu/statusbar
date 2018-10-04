#![feature(try_from)]

mod config;
mod formatter;
mod output;
mod parse;
mod statusbar;
mod util;
mod widget;

use crate::config::{Config, Format};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Write default config
    #[structopt(long = "write-default")]
    write_default: bool,

    #[structopt(short = "f", long = "format")]
    format: Option<Format>,
}

fn run() -> Result<(), failure::Error> {
    let opt = Opt::from_args();

    let cfg = if opt.write_default {
        Config::write_default()?
    } else {
        Config::load_or_write_default()?
    };

    statusbar::run(cfg, opt.format);
    Ok(())
}

fn main() {
    env_logger::init();
    if let Err(e) = run() {
        eprintln!("{}", e);
        for cause in e.iter_causes() {
            eprintln!("Caused by: {}", cause);
        }
        std::process::exit(1);
    }
}
