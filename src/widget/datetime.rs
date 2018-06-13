use chrono::{Local, Utc};

use output::Output;
use widget::Widget;

pub struct DateTimeWidget {
    cfg: Cfg,
}

impl DateTimeWidget {
    pub fn new(cfg: Cfg) -> Self {
        Self { cfg }
    }
}

impl Widget for DateTimeWidget {
    fn run(&mut self, sink: &mut Output) -> Result<(), failure::Error> {
        let fmt = match self.cfg.timezone {
            TimeZone::Local => Local::now().format(&self.cfg.format),
            TimeZone::UTC => Utc::now().format(&self.cfg.format),
        };

        sink.write(format_args!("{}", fmt));

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TimeZone {
    Local,
    UTC,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cfg {
    pub timezone: TimeZone,
    pub format: String,
}
