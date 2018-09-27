use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

use failure;
use serde_derive::{Deserialize, Serialize};

use crate::output::Output;
use crate::widget;

pub struct Widget {
    cfg: Cfg,
}

impl Widget {
    pub fn new(mut cfg: Cfg) -> Self {
        cfg.bat_name = PathBuf::from("/sys/class/power_supply")
            .join(cfg.bat_name)
            .join("uevent");
        Self { cfg }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Status {
    Unknown,
    Charging,
    Discharging,
}

impl ::std::str::FromStr for Status {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Unknown" => Ok(Status::Unknown),
            "Discharging" => Ok(Status::Discharging),
            "Charging" => Ok(Status::Charging),
            _ => Err(()),
        }
    }
}

struct UeventBat {
    power_supply_status: Status,
    power_supply_energy_full: u64,
    power_supply_energy_now: u64,
}

#[inline]
fn split_uevent(ev: &str) -> Option<(&str, &str)> {
    let mut it = ev.splitn(2, '=');
    Some((it.next()?, it.next()?))
}

fn parse_uevent<R>(mut r: R) -> Option<UeventBat>
where
    R: io::BufRead,
{
    let mut status = None;
    let mut full = None;
    let mut now = None;
    let mut ln = String::new();
    while r.read_line(&mut ln).ok()? != 0 {
        if ln.ends_with('\n') {
            ln.pop();
        }

        let (key, val) = split_uevent(&ln)?;

        if key == "POWER_SUPPLY_ENERGY_NOW" {
            now = Some(val.parse().ok()?);
        } else if key == "POWER_SUPPLY_ENERGY_FULL" {
            full = Some(val.parse().ok()?);
        } else if key == "POWER_SUPPLY_STATUS" {
            status = Some(val.parse().ok()?);
        }

        ln.clear();
    }

    Some(UeventBat {
        power_supply_status: status?,
        power_supply_energy_full: full?,
        power_supply_energy_now: now?,
    })
}

impl widget::Widget for Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error> {
        if let Ok(fh) = File::open(&self.cfg.bat_name).map(BufReader::new) {
            let _uevent_bat = parse_uevent(fh).unwrap();
        } else {
            sink.write(format_args!("{}", self.cfg.format_no_bat));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Cfg {
    pub bat_name: PathBuf,
    pub format: String,
    pub format_no_bat: String,
    pub sym_charging: String,
    pub sym_unknown: String,
    pub sym_discharging: String,
    pub good_tresh: f64,
    pub mediocre_tresh: f64,
    pub bad_tresh: f64,
}
