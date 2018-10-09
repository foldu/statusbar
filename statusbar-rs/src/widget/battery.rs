use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

use failure::{self, format_err};
use serde_derive::{Deserialize, Serialize};

use crate::{
    formatter::{Format, FormatMap},
    output::{Color, Output},
    widget,
};

pub struct Widget {
    cfg: Cfg,
    fmt_map: FormatMap,
    buf: String,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Self {
        Self {
            cfg: Cfg {
                bat_name: PathBuf::from("/sys/class/power_supply")
                    .join(cfg.bat_name)
                    .join("uevent"),
                ..cfg
            },
            fmt_map: FormatMap::new(),
            buf: String::new(),
        }
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
            let uevent = parse_uevent(fh).unwrap();

            let charge = uevent.power_supply_energy_now as f64
                / uevent.power_supply_energy_full as f64
                * 100.0;

            let color = if charge <= self.cfg.bad_treshold {
                Color::Bad
            } else if charge <= self.cfg.mediocre_treshold {
                Color::Mediocre
            } else {
                Color::Good
            };

            let sym = match uevent.power_supply_status {
                Status::Unknown => &self.cfg.sym_unknown,
                Status::Charging => &self.cfg.sym_charging,
                Status::Discharging => &self.cfg.sym_discharging,
            };

            self.fmt_map
                .update_string_with("sym", |s| s.clone_from(sym));
            self.fmt_map.insert("charge", charge);
            self.buf.clear();
            self.cfg.format.fmt(&mut self.buf, &self.fmt_map)?;

            // FIXME:
            sink.write_colored(color, format_args!("{}", self.buf));

            Ok(())
        } else {
            Err(format_err!(
                "Can't open battery in {}",
                self.cfg.bat_name.display()
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cfg {
    pub bat_name: PathBuf,
    pub format: Format,
    pub format_no_bat: Format,
    pub sym_charging: String,
    pub sym_unknown: String,
    pub sym_discharging: String,
    pub mediocre_treshold: f64,
    pub bad_treshold: f64,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            bat_name: PathBuf::from("BAT0"),
            format: Format::parse("bat: {sym}{charge:2}%").unwrap(),
            format_no_bat: Format::parse("bat: no").unwrap(),
            sym_charging: "+".to_owned(),
            sym_discharging: "-".to_owned(),
            sym_unknown: "?".to_owned(),
            mediocre_treshold: 40.,
            bad_treshold: 20.,
        }
    }
}
