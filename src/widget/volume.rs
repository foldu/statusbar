#[cfg(target_os = "linux")]
pub mod alsa;

use formatter::{FormatMap, FormatString};
use serde_derive::{Deserialize, Serialize};

use crate::output::{Color, Output};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Backend {
    #[cfg(target_os = "linux")]
    Alsa(alsa::Cfg),
}

struct VolumeState {
    pub volume: f64,
    pub is_muted: bool,
}

pub struct Widget {
    fmt_map: FormatMap,
    format: FormatString,
    format_muted: FormatString,
    mixer: Box<dyn Mixer>,
}

trait Mixer {
    fn get_volume_state(&mut self) -> Result<VolumeState, failure::Error>;
}

impl Widget {
    pub fn new(cfg: Cfg) -> Result<Self, failure::Error> {
        Ok(Self {
            fmt_map: FormatMap::new(),
            format: FormatString::parse_with_allowed_keys(&cfg.format, &["volume"])?,
            format_muted: FormatString::parse_with_allowed_keys(&cfg.format_muted, &["volume"])?,
            mixer: match cfg.backend {
                #[cfg(target_os = "linux")]
                Backend::Alsa(cfg) => Box::new(alsa::AlsaMixer::new(cfg)?),
            },
        })
    }
}

impl super::Widget for Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error> {
        let state = self.mixer.get_volume_state()?;

        self.fmt_map.insert("volume", state.volume);

        if state.is_muted {
            sink.write_colored(
                Color::Mediocre,
                format_args!("{}", self.format_muted.fmt(&self.fmt_map)?),
            );
        } else {
            sink.write(format_args!("{}", self.format.fmt(&self.fmt_map)?));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cfg {
    pub format: String,
    pub format_muted: String,
    pub backend: Backend,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            format: "vol: {volume}%".to_owned(),
            format_muted: "vol: muted".to_owned(),

            backend: Backend::Alsa(alsa::Cfg::default()),
        }
    }
}
