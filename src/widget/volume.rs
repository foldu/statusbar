// FIXME: this entire thing
use alsa::{
    mixer::{Selem, SelemChannelId, SelemId},
    Mixer,
};
use failure::format_err;
use serde_derive::{Deserialize, Serialize};

use crate::{
    formatter::{Format, FormatMap},
    output::{Color, Output},
    widget,
};

struct AlsaConn {
    vol_range: (i64, i64),
    has_playback_switch: bool,
    has_playback_volume: bool,
}

fn connect_mixer(mixer_name: &str) -> Result<Mixer, failure::Error> {
    Mixer::new(mixer_name, false).map_err(|e| format_err!("Can't open mixer {}: {}", mixer_name, e))
}

impl AlsaConn {
    fn connect(mixer_name: &str, device: &str, mixer_index: u32) -> Result<Self, failure::Error> {
        let mixer = connect_mixer(mixer_name)?;
        let ret = {
            let mut ret = Self {
                vol_range: (0, 0),
                has_playback_switch: false,
                has_playback_volume: false,
            };
            let selem = ret.get_selem(&mixer, &device, mixer_index)?;
            let vol_range = selem.get_playback_volume_range();
            let has_playback_switch = selem.has_playback_switch();
            let has_playback_volume = selem.has_playback_volume();

            ret.has_playback_switch = has_playback_switch;
            ret.has_playback_volume = has_playback_volume;
            ret.vol_range = vol_range;

            ret
        };

        Ok(ret)
    }

    pub fn get_selem<'a>(
        &self,
        mixer: &'a Mixer,
        device: &str,
        mixer_index: u32,
    ) -> Result<Selem<'a>, failure::Error> {
        mixer
            .find_selem(&SelemId::new(device, mixer_index))
            .ok_or_else(|| {
                format_err!(
                    "Can't find selem with name {} and index {}",
                    device,
                    mixer_index
                )
            })
    }

    pub fn get_volume<'a>(
        &self,
        selem: &'a Selem,
        channel: SelemChannelId,
    ) -> Result<i64, failure::Error> {
        if self.has_playback_volume {
            let vol = selem.get_playback_volume(channel)?;
            Ok(vol * 100 / self.vol_range.1)
        } else {
            Ok(100)
        }
    }

    pub fn is_muted<'a>(
        &self,
        selem: &'a Selem,
        channel: SelemChannelId,
    ) -> Result<bool, failure::Error> {
        Ok(self.has_playback_switch && selem.get_playback_switch(channel)? == 0)
    }
}

pub struct Widget {
    cfg: Cfg,
    conn: Option<AlsaConn>,
    map: FormatMap,
    buf: String,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Self {
        let conn = AlsaConn::connect(&cfg.mixer, &cfg.device, cfg.mixer_index).ok();
        Self {
            conn,
            cfg,
            map: FormatMap::new(),
            buf: String::new(),
        }
    }
}

impl widget::Widget for Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error> {
        if let Some(ref mut conn) = self.conn {
            let mixer = connect_mixer(&self.cfg.mixer)?;
            let selem = conn.get_selem(&mixer, &self.cfg.device, self.cfg.mixer_index)?;
            let volume = conn.get_volume(&selem, self.cfg.channel_id)?;
            let is_muted = conn.is_muted(&selem, self.cfg.channel_id)?;

            self.buf.clear();
            self.map.insert("volume", volume);

            if is_muted {
                self.cfg.format_muted.fmt(&mut self.buf, &self.map)?;
                sink.write_colored(Color::Mediocre, format_args!("{}", self.buf));
            } else {
                self.cfg.format.fmt(&mut self.buf, &self.map)?;
                sink.write(format_args!("{}", self.buf));
            }
        } else {
            let conn = AlsaConn::connect(&self.cfg.mixer, &self.cfg.device, self.cfg.mixer_index)?;
            self.conn = Some(conn);
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(remote = "SelemChannelId")]
enum SelemChannelIdDef {
    Unknown,
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
    FrontCenter,
    Woofer,
    SideLeft,
    SideRight,
    RearCenter,
    Last,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cfg {
    pub mixer: String,
    pub device: String,
    pub mixer_index: u32,
    #[serde(with = "SelemChannelIdDef")]
    pub channel_id: SelemChannelId,
    pub format: Format,
    pub format_muted: Format,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            mixer: "hw:0".to_owned(),
            device: "Master".to_owned(),
            mixer_index: 0,
            channel_id: SelemChannelId::FrontLeft,
            format: Format::parse("vol: {volume}%").unwrap(),
            format_muted: Format::parse("vol: muted").unwrap(),
        }
    }
}
