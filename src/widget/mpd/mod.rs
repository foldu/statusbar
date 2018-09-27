mod conn;

use serde_derive::{Deserialize, Serialize};

use self::conn::{MpdConnection, MpdStatus};
use crate::output::Output;
use crate::widget;

pub struct Widget {
    cfg: Cfg,
    conn: Option<MpdConnection>,
    status: MpdStatus,
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
            format_running: "mpd: {artist} - {title}".into(),
            format_stopped: "mpd: stopped".into(),
            format_paused: "mpd: paused {artist} - {title}".into(),
            format_down: "mpd: ded".into(),
            endpoint: "localhost:6600".into(),
        }
    }
}

impl widget::Widget for Widget {
    fn run(&mut self, _sink: &mut dyn Output) -> Result<(), failure::Error> {
        Ok(())
    }
}
