#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use self::linux::get_cpu_usage;
use crate::output::Output;

use formatter::{FormatMap, FormatString};
use serde_derive::{Deserialize, Serialize};

pub struct Widget {
    fmt_map: FormatMap,
    format: FormatString,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Result<Self, failure::Error> {
        Ok(Self {
            fmt_map: FormatMap::new(),
            format: FormatString::parse_with_allowed_keys(&cfg.format, &["usage"])?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cfg {
    format: String,
}

impl super::Widget for Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error> {
        self.fmt_map.insert("usage", get_cpu_usage());
        sink.write(format_args!("{}", self.format.fmt(&self.fmt_map)?));
        Ok(())
    }
}
