use std::fs;
use std::io;
use std::path::PathBuf;

use output::OutputKind;
use widget::{battery, datetime, mpd, WidgetKind};

use directories::BaseDirs;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Can't load config: {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "Can't load config: {}", _0)]
    RonSer(#[cause] ron::ser::Error),
    #[fail(display = "Can't load config: {}", _0)]
    RonDe(#[cause] ron::de::Error),
}

impl From<ron::ser::Error> for Error {
    fn from(e: ron::ser::Error) -> Self {
        Error::RonSer(e)
    }
}

impl From<ron::de::Error> for Error {
    fn from(e: ron::de::Error) -> Self {
        Error::RonDe(e)
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub general: GeneralCfg,
    pub format: OutputKind,
    pub widgets: Vec<WidgetKind>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralCfg {
                color: true,
                update_interval: 1000,
            },
            format: OutputKind::default(),
            widgets: vec![
                WidgetKind::Datetime(datetime::Cfg::default()),
                WidgetKind::Battery(battery::Cfg::default()),
                WidgetKind::Mpd(mpd::Cfg::default()),
            ],
        }
    }
}

lazy_static! {
    pub static ref CONFIG_PATH: PathBuf = {
        BaseDirs::new()
            .unwrap()
            .config_dir()
            .join("statusbar-rs.ron")
    };
}

impl Config {
    #[inline]
    pub fn load() -> Result<Self, Error> {
        ron::de::from_str(&fs::read_to_string(&*CONFIG_PATH)?).map(Ok)?
    }

    pub fn write_default() -> Result<Self, Error> {
        let ret = Self::default();
        fs::write(
            &*CONFIG_PATH,
            &ron::ser::to_string_pretty(&ret, ron::ser::PrettyConfig::default()).unwrap(),
        )?;
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

#[test]
fn default_config_works() {
    let def = Config::default();
    assert!(ron::ser::to_string(&def).is_ok());
}
