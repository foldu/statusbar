mod conn;

use formatter::{FormatMap, FormatString};
use serde_derive::{Deserialize, Serialize};

use self::conn::{MpdConnection, MpdStatus};
use crate::{
    output::{Color, Output},
    widget,
};

pub struct Widget {
    cfg: Cfg,
    conn: Option<MpdConnection>,
    status: MpdStatus,
    buf: String,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Self {
        Self {
            conn: match MpdConnection::connect(&cfg.endpoint) {
                Ok(conn) => Some(conn),
                // FIXME: TODO: return failure::Error instead of panicking
                Err(conn::Error::NotMpd) => panic!("Not mpd"),
                _ => None,
            },
            cfg,
            buf: String::new(),
            status: MpdStatus::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cfg {
    pub format_running: String,
    pub format_paused: String,
    pub format_stopped: String,
    pub format_down: String,
    pub endpoint: String,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            format_running: "mpd: {artist} - {title}".parse().unwrap(),
            format_stopped: "mpd: stopped".parse().unwrap(),
            format_paused: "mpd: paused {artist} - {title}".parse().unwrap(),
            format_down: "mpd: ded".parse().unwrap(),
            endpoint: "localhost:6600".parse().unwrap(),
        }
    }
}

impl widget::Widget for Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error> {
        if let Some(ref conn) = self.conn {
        } else {
        }
        Ok(())
    }
}
