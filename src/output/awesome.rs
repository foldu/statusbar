use std::fmt::{self, Write};

use serde_derive::{Deserialize, Serialize};

use super::HexRgb;
use crate::output::{Color, GColors};

#[derive(Debug, Clone)]
pub struct Output {
    buf: String,
    cfg: Cfg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cfg {
    separator: String,
    separator_color: HexRgb,
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
            separator: " | ".into(),
            separator_color: "#333333".parse().unwrap(),
        }
    }
}

impl Output {
    pub fn new(cfg: Cfg) -> Self {
        Self {
            buf: String::new(),
            cfg,
        }
    }
}

impl super::Output for Output {
    fn start(&mut self) {
        self.buf.clear();
    }

    fn write(&mut self, s: fmt::Arguments) {
        write!(self.buf, "{}", s).unwrap()
    }

    fn write_colored(&mut self, c: Color, s: fmt::Arguments) {
        let color = match c {
            Color::Good => &self.cfg.colors.good.0,
            Color::Mediocre => &self.cfg.colors.mediocre.0,
            Color::Bad => &self.cfg.colors.bad.0,
        };
        write!(self.buf, "<span color=\"{}\">{}</span>", color, s).unwrap()
    }

    fn write_sep(&mut self) {
        write!(
            self.buf,
            "<span color=\"{}\">{}</span>",
            self.cfg.separator_color.0, self.cfg.separator
        )
        .unwrap()
    }

    fn finish(&mut self) {
        println!("{}", self.buf);
    }
}
