use std::{
    io::{self, prelude::*, BufReader},
    net::{TcpStream, ToSocketAddrs},
    str,
    time::Duration,
};

use failure::Fail;

use crate::util::{str_copy, OptionalString};

pub struct MpdConnection {
    sock: BufReader<TcpStream>,
    buf: Vec<u8>,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Server is not mpd")]
    NotMpd,
    #[fail(display = "Error while communicating with mpd")]
    Io(#[cause] io::Error),
    #[fail(display = "Got unparseable mpd output: {}", _0)]
    InvalidMpdOutput(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

#[derive(Debug, Default, Clone)]
pub struct MpdStatus {
    pub volume: i32,
    pub elapsed: u64,
    pub duration: u64,
    pub artist: OptionalString,
    pub file: String,
    pub title: OptionalString,
    pub paused: bool,
}

#[derive(Debug, Default, Clone)]
pub struct MpdState {
    stopped: bool,
    inner: MpdStatus,
}

impl MpdState {
    pub fn get(&self) -> Option<&MpdStatus> {
        if self.stopped {
            None
        } else {
            Some(&self.inner)
        }
    }
}

const READ_TIMEOUT: Duration = Duration::from_millis(100);

impl MpdConnection {
    pub fn connect<A>(addr: A) -> Result<Self, Error>
    where
        A: ToSocketAddrs,
    {
        let mut ret = MpdConnection {
            sock: TcpStream::connect(addr).map(BufReader::new)?,
            buf: Vec::new(),
        };

        ret.sock.get_mut().set_read_timeout(Some(READ_TIMEOUT))?;

        let mut buf = [0u8; 32];

        let nbytes = ret.sock.read(&mut buf)?;
        if !buf[..nbytes].starts_with(b"OK MPD ") {
            Err(Error::NotMpd)
        } else {
            Ok(ret)
        }
    }

    pub fn get_status(&mut self, status: &mut MpdState) -> Result<(), Error> {
        self.send_command_with("status\n", |key, val| {
            if key == "volume" {
                status.inner.volume = val.parse().ok()?;
            } else if key == "elapsed" {
                status.inner.elapsed = val.parse::<f64>().ok()?.floor() as u64;
            } else if key == "duration" {
                status.inner.duration = val.parse::<f64>().ok()?.floor() as u64;
            } else if key == "state" {
                if val == "stop" {
                    status.stopped = true;
                } else {
                    status.stopped = false;
                    status.inner.paused = val == "pause";
                }
            }
            Some(())
        })?;

        if status.stopped {
            return Ok(());
        }

        status.inner.title.invalidate();
        status.inner.artist.invalidate();

        self.send_command_with("currentsong\n", |key, val| {
            if key == "file" {
                str_copy(&mut status.inner.file, val);
            } else if key == "Artist" {
                status.inner.artist.copy_from(val);
            } else if key == "Title" {
                status.inner.title.copy_from(val);
            }
            Some(())
        })?;

        Ok(())
    }

    fn send_command_with<F>(&mut self, cmd: &str, mut f: F) -> Result<(), Error>
    where
        F: FnMut(&str, &str) -> Option<()>,
    {
        assert!(cmd.ends_with('\n'));
        self.sock.get_mut().write_all(cmd.as_bytes())?;

        loop {
            self.buf.clear();
            self.sock.read_until(b'\n', &mut self.buf)?;
            self.buf.pop();
            if self.buf == b"OK" {
                break;
            }

            let s = String::from_utf8_lossy(&self.buf);
            let mut it = s.splitn(2, ": ");
            let mut next_field = || {
                it.next()
                    .ok_or_else(|| Error::InvalidMpdOutput(s.to_string()))
            };
            if f(next_field()?, next_field()?).is_none() {
                return Err(Error::InvalidMpdOutput(s.to_string()));
            }
        }

        Ok(())
    }
}
