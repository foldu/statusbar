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
    power_supply_full: u64,
    power_supply_now: u64,
}

macro_rules! mk_power_supply_parser {
    ($fn:ident, $id:ident) => {
        fn $fn(r: &mut impl io::BufRead, buf: &mut String, uevent: &mut UeventBat) -> Option<()> {
            let mut field = 0_u8;
            while r.read_line(buf).ok()? != 0 {
                if buf.ends_with('\n') {
                    buf.pop();
                }

                let (key, val) = split_uevent(&buf)?;

                if key == concat!("POWER_SUPPLY_", stringify!($id), "_NOW") {
                    uevent.power_supply_now = val.parse().ok()?;
                    field |= 1;
                } else if key == concat!("POWER_SUPPLY_", stringify!($id), "_FULL") {
                    uevent.power_supply_full = val.parse().ok()?;
                    field |= 2;
                } else if key == concat!("POWER_SUPPLY_", stringify!($id), "_NOW") {
                    uevent.power_supply_status = val.parse().ok()?;
                    field |= 4;
                }

                buf.clear();
            }
            if field == 1 | 2 | 4 {
                Some(())
            } else {
                None
            }
        }
    };
}

mk_power_supply_parser!(parse_with_energy, ENERGY);
mk_power_supply_parser!(parse_with_charge, CHARGE);

#[inline]
fn split_uevent(ev: &str) -> Option<(&str, &str)> {
    let mut it = ev.splitn(2, '=');
    Some((it.next()?, it.next()?))
}

fn parse_uevent<R>(mut r: R) -> Option<UeventBat>
where
    R: io::BufRead + io::Seek,
{
    let mut uevent = UeventBat {
        power_supply_full: 1,
        power_supply_now: 1,
        power_supply_status: Status::Unknown,
    };
    let mut buf = String::new();

    if let Some(_) = parse_with_energy(&mut r, &mut buf, &mut uevent) {
        return Some(uevent);
    }

    r.seek(io::SeekFrom::Start(0)).ok()?;

    parse_with_charge(&mut r, &mut buf, &mut uevent).map(|_| uevent)
}

impl widget::Widget for Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error> {
        if let Ok(fh) = File::open(&self.bat_path).map(BufReader::new) {
            let uevent = parse_uevent(fh).unwrap();

            let charge = uevent.power_supply_now as f64 / uevent.power_supply_full as f64 * 100.0;

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
