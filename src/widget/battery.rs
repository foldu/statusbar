use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

use failure::{self, format_err};
use formatter::{FormatMap, FormatString};
use serde_derive::{Deserialize, Serialize};

use crate::{
    output::{Color, Output},
    widget,
};

pub struct Widget {
    fmt_map: FormatMap,
    bat_path: PathBuf,
    format: FormatString,
    sym_charging: String,
    sym_unknown: String,
    sym_discharging: String,
    mediocre_treshold: f64,
    bad_treshold: f64,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Result<Self, failure::Error> {
        let bat_path = PathBuf::from("/sys/class/power_supply")
            .join(&cfg.bat_name)
            .join("uevent");

        if !bat_path.exists() {
            return Err(format_err!("Battery named {} doesn't exist", cfg.bat_name));
        }

        Ok(Self {
            fmt_map: FormatMap::new(),
            bat_path,
            bad_treshold: cfg.bad_treshold,
            sym_charging: cfg.sym_charging,
            sym_discharging: cfg.sym_discharging,
            mediocre_treshold: cfg.mediocre_treshold,
            format: FormatString::parse_with_allowed_keys(&cfg.format, &["sym", "charge"])?,
            sym_unknown: cfg.sym_unknown,
        })
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

#[derive(Debug, Clone, Copy)]
struct UeventBat {
    power_supply_status: Status,
    charge_percent: f64,
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
    let mut status = Status::Unknown;
    let mut buf = String::new();

    let mut energy: (Option<u64>, Option<u64>) = (None, None);
    let mut charge = (None, None);
    let mut capacity = None;

    while r.read_line(&mut buf).ok()? != 0 {
        if buf.ends_with('\n') {
            buf.pop();
        }

        let (key, val) = split_uevent(&buf)?;

        // duplicated *ENERGY* and *CHARGE* are a fix against buggy linux drivers that swap ENERGY and CHARGE
        // when they feel like it even when CHARGE is something completely different than ENERGY
        if key == "POWER_SUPPLY_ENERGY_NOW" {
            energy.0 = val.parse().ok();
        } else if key == "POWER_SUPPLY_ENERGY_FULL" {
            energy.1 = val.parse().ok();
        } else if key == "POWER_SUPPLY_CHARGE_NOW" {
            charge.0 = val.parse().ok();
        } else if key == "POWER_SUPPLY_CHARGE_FULL" {
            charge.1 = val.parse().ok();
        } else if key == "POWER_SUPPLY_STATUS" {
            status = val.parse().ok()?;
        } else if key == "POWER_SUPPLY_CHARGE_CAPACITY" {
            capacity = val.parse().ok();
        }

        buf.clear();
    }

    match (capacity, energy, charge) {
        // ENERGY has highest priority, when we didn't get energy fall back to CHARGE
        (_, (Some(now), Some(full)), _) | (_, _, (Some(now), Some(full))) => Some(UeventBat {
            power_supply_status: status,
            charge_percent: now as f64 / full as f64 * 100.0,
        }),
        // otherwise fall back to capacity
        (Some(capacity), _, _) => Some(UeventBat {
            power_supply_status: status,
            charge_percent: capacity,
        }),
        // just give up
        _ => None,
    }
}

impl widget::Widget for Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error> {
        if let Ok(fh) = File::open(&self.bat_path).map(BufReader::new) {
            let uevent = parse_uevent(fh).unwrap();

            let charge = uevent.charge_percent;

            let color = if charge <= self.bad_treshold {
                Color::Bad
            } else if charge <= self.mediocre_treshold {
                Color::Mediocre
            } else {
                Color::Good
            };

            let sym = match uevent.power_supply_status {
                Status::Unknown => &self.sym_unknown,
                Status::Charging => &self.sym_charging,
                Status::Discharging => &self.sym_discharging,
            };

            self.fmt_map
                .update_string_with("sym", |s| s.clone_from(sym));
            self.fmt_map.insert("charge", charge);

            sink.write_colored(color, format_args!("{}", self.format.fmt(&self.fmt_map)?));

            Ok(())
        } else {
            Err(format_err!(
                "Can't open battery in {}",
                self.bat_path.display()
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cfg {
    pub bat_name: String,
    pub format: String,
    pub sym_charging: String,
    pub sym_unknown: String,
    pub sym_discharging: String,
    pub mediocre_treshold: f64,
    pub bad_treshold: f64,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            bat_name: "BAT0".to_owned(),
            format: "bat: {sym}{charge:.2}%".to_owned(),
            sym_charging: "+".to_owned(),
            sym_discharging: "-".to_owned(),
            sym_unknown: "?".to_owned(),
            mediocre_treshold: 40.,
            bad_treshold: 20.,
        }
    }
}
