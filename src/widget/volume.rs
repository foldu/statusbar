use alsa::{
    mixer::{Selem, SelemId},
    Mixer,
};
use failure::format_err;

use crate::output::Output;
use crate::widget;

struct AlsaConn {
    mixer: Mixer,
    // self referential struct
    // selem: Selem<'a>,
    // vol_range: (i64, i64),
}

impl AlsaConn {
    fn connect(mixer_name: &str) -> Result<Self, failure::Error> {
        let mixer = Mixer::new(mixer_name, false)
            .map_err(|e| format_err!("Can't open mixer {}: {}", mixer_name, e))?;
        Ok(Self { mixer: mixer })
    }

    fn get_selem(&self, device: &str, mixer_index: u32) -> Result<Selem, failure::Error> {
        self.mixer
            .find_selem(&SelemId::new(device, mixer_index))
            .ok_or_else(|| {
                format_err!(
                    "Can't find selem with name {} and index {}",
                    device,
                    mixer_index
                )
            })
    }
}

struct Widget {
    cfg: Cfg,
    conn: AlsaConn,
    volume_range: (i64, i64),
}

impl Widget {
    pub fn new(cfg: Cfg) -> Result<Self, failure::Error> {
        let conn = AlsaConn::connect(&cfg.mixer)?;
        Ok(Self {
            volume_range: conn
                .get_selem(&cfg.device, cfg.mixer_index)?
                .get_playback_volume_range(),
            cfg,
            conn,
        })
    }
}

impl widget::Widget for Widget {
    fn run(&mut self, _sink: &mut dyn Output) -> Result<(), failure::Error> {
        Ok(())
    }
}

pub struct Cfg {
    pub mixer: String,
    pub device: String,
    pub mixer_index: u32,
    pub format: String,
}
