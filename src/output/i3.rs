use std::fmt::{self, Write};

use serde_derive::{Deserialize, Serialize};

use super::HexRgb;
use crate::output::{Color, GColors};

pub struct Output {
    buf: String,
    cfg: Cfg,
}

impl Output {
    pub fn new(cfg: Cfg) -> Self {
        Self {
            buf: String::new(),
            cfg,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cfg {
    colors: GColors<HexRgb>,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            colors: GColors {
                good: "#00FF00".parse().unwrap(),
                bad: "#FF0000".parse().unwrap(),
                mediocre: "#FFFF00".parse().unwrap(),
            },
        }
    }
}

impl super::Output for Output {
    fn init(&mut self) {
        println!(r#"{{"version":1}}"#);
        println!("[");
    }

    fn start(&mut self) {
        self.buf.clear();
        print!("[");
    }

    fn write(&mut self, s: fmt::Arguments) {
        write!(self.buf, r#"{{"full_text": "{}"}},"#, s).unwrap()
    }

    fn write_colored(&mut self, c: Color, s: fmt::Arguments) {
        let color = match c {
            Color::Good => &self.cfg.colors.good.0,
            Color::Mediocre => &self.cfg.colors.mediocre.0,
            Color::Bad => &self.cfg.colors.bad.0,
        };
        write!(
            self.buf,
            r#"{{"full_text": "{}","color": "{}"}},"#,
            s, color
        )
        .unwrap()
    }

    fn write_sep(&mut self) {}

    fn finish(&mut self) {
        if self.buf.ends_with(",") {
            self.buf.pop();
        }
        println!("{}],", self.buf);
    }
}
