use chrono::{Local, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::{output::Output, widget};

pub struct Widget {
    cfg: Cfg,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Self {
        Self { cfg }
    }
}

impl widget::Widget for Widget {
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
