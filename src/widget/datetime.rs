use chrono::{Local, Utc};

use output::Output;
use widget::Widget;

pub struct DatetimeWidget {
    cfg: Cfg,
}

impl DatetimeWidget {
    pub fn new(cfg: Cfg) -> Self {
        Self { cfg }
    }
}

impl Widget for DatetimeWidget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error> {
        let fmt = match self.cfg.timezone {
            Timezone::Local => Local::now().format(&self.cfg.format),
            Timezone::UTC => Utc::now().format(&self.cfg.format),
        };

        sink.write(format_args!("{}", fmt));

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Timezone {
    Local,
    UTC,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Cfg {
    pub timezone: Timezone,
    pub format: String,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            format: "%Y-%m-%d %H:%M:%S".into(),
            timezone: Timezone::Local,
        }
    }
}
