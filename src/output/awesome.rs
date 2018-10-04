use std::fmt::{self, Write};

use serde_derive::{Deserialize, Serialize};

use super::color::{ColorCfg, GColors, HexRgb};
use crate::output::Color;

#[derive(Debug, Clone)]
pub struct Output {
    buf: String,
    cfg: Cfg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cfg {
    separator: String,
    colors: GColors<HexRgb>,
}

impl Output {
    pub fn new(sep: &str, colors: &ColorCfg) -> Self {
        Self {
            buf: String::new(),
            cfg: Cfg {
                separator: sep.to_owned(),
                colors: colors.hex.clone(),
            },
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
            Color::Good => self.cfg.colors.good.as_ref(),
            Color::Mediocre => self.cfg.colors.mediocre.as_ref(),
            Color::Bad => self.cfg.colors.bad.as_ref(),
        };
        write!(self.buf, "<span color=\"{}\">{}</span>", color, s).unwrap()
    }

    fn write_sep(&mut self) {
        write!(
            self.buf,
            "<span color=\"{}\">{}</span>",
            self.cfg.colors.separator.as_ref(),
            self.cfg.separator
        )
        .unwrap()
    }

    fn finish(&mut self) {
        println!("{}", self.buf);
    }
}
