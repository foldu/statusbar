#![feature(try_from)]

mod config;
mod formatter;
mod output;
mod parse;
mod statusbar;
mod util;
mod widget;

use crate::config::{Config, DefaultConfig};

use failure::format_err;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Write default config
    #[structopt(long = "write-default")]
    write_default: Option<String>,
}

fn run() -> Result<(), failure::Error> {
    let opt = Opt::from_args();

    let cfg = match opt.write_default {
        None => Config::load_or_write_default(DefaultConfig::Terminal)?,
        Some(s) => {
            // FIXME: put me somewhere else
            // or wait for clap v3 with serde support
            let def = match s.as_str() {
                "awesome" => DefaultConfig::Awesome,
                "terminal" => DefaultConfig::Terminal,
                _ => return Err(format_err!("Invalid output format: {}", s)),
            };

            Config::write_default(def)?
        }
    };

    statusbar::run(cfg);
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
