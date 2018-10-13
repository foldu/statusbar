mod conn;

use std::cell::RefCell;

use failure::format_err;
use formatter::{FormatMap, FormatString};
use serde_derive::{Deserialize, Serialize};

use self::conn::{MpdConnection, MpdState};
use crate::{
    output::{Color, Output},
    widget,
};

pub struct Widget {
    conn: Option<MpdConnection>,
    state: RefCell<MpdState>,
    format_running: FormatString,
    format_paused: FormatString,
    format_stopped: FormatString,
    format_down: FormatString,
    fmt_map: FormatMap,
    endpoint: String,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Result<Self, failure::Error> {
        let running_allowed_keys = ["artist", "title", "path", "elapsed", "duration"];
        Ok(Self {
            conn: match MpdConnection::connect(&cfg.endpoint) {
                Ok(conn) => Some(conn),
                Err(conn::Error::NotMpd) => Err(format_err!(
                    "mpd widget tried to connect to something that is not mpd"
                ))?,
                _ => None,
            },
            format_running: FormatString::parse_with_allowed_keys(
                &cfg.format_running,
                &running_allowed_keys,
            )?,
            format_paused: FormatString::parse_with_allowed_keys(
                &cfg.format_paused,
                &running_allowed_keys,
            )?,
            format_stopped: FormatString::parse_with_allowed_keys(&cfg.format_stopped, &[""])?,
            format_down: FormatString::parse_with_allowed_keys(&cfg.format_down, &[""])?,
            state: RefCell::new(MpdState::default()),
            endpoint: cfg.endpoint,
            fmt_map: FormatMap::new(),
        })
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
        let mut state = self.state.borrow_mut();

        // RefCell is only needed for this nice pattern match
        // could be refactored into something else
        match self.conn.as_mut().map(|conn| conn.get_status(&mut *state)) {
            Some(Ok(())) => {
                if let Some(status) = state.get() {
                    self.fmt_map.update_string_with("artist", |s| {
                        s.push_str(status.artist.get().unwrap_or(""))
                    });

                    self.fmt_map.update_string_with("title", |s| {
                        s.push_str(status.title.get().unwrap_or(""))
                    });

                    self.fmt_map
                        .update_string_with("file", |s| s.push_str(&status.file));

                    self.fmt_map.insert("duration", status.duration as f64);

                    self.fmt_map.insert("elapsed", status.elapsed as f64);

                    let fmt = if status.paused {
                        &self.format_paused
                    } else {
                        &self.format_running
                    };

                    sink.write(format_args!("{}", fmt.fmt(&self.fmt_map)?));
                } else {
                    sink.write(format_args!("{}", self.format_stopped.fmt(&self.fmt_map)?))
                }
            }

            Some(Err(e)) => match e {
                // on programming error or on something that doesn't look like mpd
                conn::Error::NotMpd => {
                    self.conn = None;
                    return Err(e.into());
                }

                conn::Error::InvalidMpdOutput(_) => {
                    self.conn = None;
                    return Err(e.into());
                }

                conn::Error::Io(ref io_e) => {
                    use std::io;
                    match io_e.kind() {
                        // don't do anything if mpd just failed to respond in time
                        // note: socket read returns EAGAIN on read timeout instead of ETIMEOUT
                        io::ErrorKind::Interrupted => {}
                        // otherwise report the error
                        _ => {
                            self.conn = None;
                            return Err(e.into());
                        }
                    }
                }
            },

            None => {
                // try reconnecting
                if let Ok(conn) = MpdConnection::connect(&self.endpoint) {
                    self.conn = Some(conn);
                }
                sink.write_colored(
                    Color::Bad,
                    format_args!("{}", self.format_down.fmt(&self.fmt_map)?),
                )
            }
        }
        Ok(())
    }
}
