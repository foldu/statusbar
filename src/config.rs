use std::{fs, io, path::PathBuf};

use directories::BaseDirs;
use failure::Fail;
use lazy_static::*;
use serde_derive::{Deserialize, Serialize};

use crate::{
    output::{default_output, OutputKind},
    widget::{battery, datetime, mpd, net, volume, WidgetKind},
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum DefaultConfig {
    Awesome,
    Terminal,
    I3,
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
    pub update_interval: u32,
    pub enable_desktop_notifications: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub general: GeneralCfg,
    pub format: OutputKind,
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

    fn with_default_config(def: DefaultConfig) -> (String, Self) {
        let ret = Self {
            general: GeneralCfg {
                color: true,
                update_interval: 1000,
                enable_desktop_notifications: true,
            },
            format: default_output(def),
            widgets: vec![
                WidgetKind::Mpd(mpd::Cfg::default()),
                WidgetKind::Net(net::Cfg::default()),
                WidgetKind::Battery(battery::Cfg::default()),
                WidgetKind::Volume(volume::Cfg::default()),
                WidgetKind::Datetime(datetime::Cfg::default()),
            ],
        };
        (toml::to_string_pretty(&ret).unwrap(), ret)
    }

    pub fn write_default(def: DefaultConfig) -> Result<Self, Error> {
        let (cont, ret) = Self::with_default_config(def);
        fs::write(
            &*CONFIG_PATH,
            &cont
            //&ron::ser::to_string_pretty(&ret, ron::ser::PrettyConfig::default()).unwrap(),
        )?;
        Ok(ret)
    }

    pub fn load_or_write_default(def: DefaultConfig) -> Result<Self, Error> {
        match Self::load() {
            Ok(cfg) => Ok(cfg),
            Err(Error::Io(io_e)) => {
                use std::io::ErrorKind;
                if let ErrorKind::NotFound = io_e.kind() {
                    Self::write_default(def)
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
        Config::with_default_config(DefaultConfig::Terminal);
        Config::with_default_config(DefaultConfig::Awesome);
    }
}
