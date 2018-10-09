use std::{fs, io, path::PathBuf};

use directories::BaseDirs;
use failure::{format_err, Fail};
use lazy_static::*;
use serde_derive::{Deserialize, Serialize};

use crate::{
    output::ColorCfg,
    widget::{battery, datetime, memory, mpd, net, temp, volume, WidgetKind},
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Awesome,
    Terminal,
    I3,
}

impl std::str::FromStr for Format {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "awesome" => Ok(Format::Awesome),
            "terminal" => Ok(Format::Terminal),
            "i3" => Ok(Format::I3),
            _ => Err(format_err!(
                "Invalid format specifier: {}, accepted formats: awesome, terminal, i3",
                s
            )),
        }
    }
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Can't open config")]
    Io(#[cause] io::Error),
    #[fail(display = "Can't deserialize config")]
    Toml(#[cause] toml::de::Error),
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::Toml(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralCfg {
    pub color: bool,
    pub default_output_format: Format,
    pub update_interval: u32,
    pub enable_desktop_notifications: bool,
    pub separator: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub general: GeneralCfg,
    pub colors: ColorCfg,
    pub widgets: Vec<WidgetKind>,
}

lazy_static! {
    pub static ref CONFIG_PATH: PathBuf = {
        BaseDirs::new()
            .unwrap()
            .config_dir()
            .join("statusbar-rs.toml")
    };
}

impl Config {
    #[inline]
    pub fn load() -> Result<Self, Error> {
        toml::from_str(&fs::read_to_string(&*CONFIG_PATH)?).map(Ok)?
    }

    fn default() -> (String, Self) {
        let ret = Self {
            general: GeneralCfg {
                color: true,
                update_interval: 1000,
                enable_desktop_notifications: true,
                separator: " | ".to_owned(),
                default_output_format: Format::Terminal,
            },
            colors: ColorCfg::default(),
            widgets: vec![
                WidgetKind::Temp(temp::Cfg::default()),
                WidgetKind::Memory(memory::Cfg::default()),
                WidgetKind::Mpd(mpd::Cfg::default()),
                WidgetKind::Net(net::Cfg::default()),
                WidgetKind::Battery(battery::Cfg::default()),
                WidgetKind::Volume(volume::Cfg::default()),
                WidgetKind::Datetime(datetime::Cfg::default()),
            ],
        };
        (toml::to_string_pretty(&ret).unwrap(), ret)
    }

    pub fn write_default() -> Result<Self, Error> {
        let (cont, ret) = Self::default();
        fs::write(&*CONFIG_PATH, &cont)?;
        Ok(ret)
    }

    pub fn load_or_write_default() -> Result<Self, Error> {
        match Self::load() {
            Ok(cfg) => Ok(cfg),
            Err(Error::Io(io_e)) => {
                use std::io::ErrorKind;
                if let ErrorKind::NotFound = io_e.kind() {
                    Self::write_default()
                } else {
                    Err(Error::Io(io_e))
                }
            }
            e => e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_works() {
        Config::default();
    }
}
