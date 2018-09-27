mod config;
mod formatter;
mod output;
mod parse;
mod statusbar;
mod util;
mod widget;

use crate::config::Config;

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

    let _cfg = if opt.write_default {
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
