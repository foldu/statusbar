mod conn;

use serde_derive::{Deserialize, Serialize};

use self::conn::{MpdConnection, MpdStatus};
use crate::{
    formatter::Format,
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
    pub format_running: Format,
    pub format_paused: Format,
    pub format_stopped: Format,
    pub format_down: Format,
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
            self.buf.clear();
            self.cfg.format_down.fmt_no_lookup(&mut self.buf);
            sink.write_colored(Color::Bad, format_args!("{}", self.buf))
        }
        Ok(())
    }
}
