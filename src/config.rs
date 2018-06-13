use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use output::AwesomeCfg;
use widget;
use widget::{battery, datetime};

use directories::BaseDirs;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Can't load config: {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "Can't load config: {}", _0)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Awesome,
    I3,
    Dzen2,
    Term,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputCfg {
    pub awesome: AwesomeCfg,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralCfg {
    pub order: Vec<widget::WidgetKind>,
    pub color: bool,
    pub update_interval: u32,
    pub output: OutputFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub general: GeneralCfg,
    pub formats: OutputCfg,
    pub battery: battery::Cfg,
    pub datetime: datetime::Cfg,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralCfg {
                order: vec![widget::WidgetKind::DateTime, widget::WidgetKind::Battery],
                color: true,
                update_interval: 1000,
                output: OutputFormat::Awesome,
            },
            formats: OutputCfg {
                awesome: AwesomeCfg::default(),
            },
            battery: battery::Cfg { test: 3 },
            datetime: datetime::Cfg {
                format: "%Y-%m-%d %H:%M:%S".into(),
                timezone: datetime::TimeZone::Local,
            },
        }
    }
}

lazy_static! {
    pub static ref CONFIG_PATH: PathBuf =
        { BaseDirs::new().config_dir().join("statusbar-rs.toml") };
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        toml::from_str(&fs::read_to_string(&*CONFIG_PATH)?).map(Ok)?
    }

    fn write_default_to<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let ret = Self::default();
        fs::write(path, toml::to_string_pretty(&ret).unwrap())?;
        Ok(ret)
    }

    pub fn write_default() -> Result<Self, Error> {
        Self::write_default_to(&*CONFIG_PATH)
    }
}

#[cfg(unix)]
#[test]
fn default_config_write_works() {
    match Config::write_default_to("/tmp/test.toml") {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            assert!(false);
        }
    }
}
