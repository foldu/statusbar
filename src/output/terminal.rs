use std::fmt::{self, Write};

use serde_derive::{Deserialize, Serialize};

use crate::output::Color;

#[derive(Debug, Clone)]
pub struct Output {
    buf: String,
    cfg: Cfg,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(remote = "console::Color")]
enum ColorDef {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

// need to duplicate instead of just using GColors because of serde proxy type
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MeColors {
    #[serde(with = "ColorDef")]
    good: console::Color,
    #[serde(with = "ColorDef")]
    mediocre: console::Color,
    #[serde(with = "ColorDef")]
    bad: console::Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cfg {
    separator: String,
    #[serde(with = "ColorDef")]
    separator_color: console::Color,
    colors: MeColors,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            colors: MeColors {
                good: console::Color::Green,
                bad: console::Color::Red,
                mediocre: console::Color::Yellow,
            },
            separator: " | ".into(),
            separator_color: console::Color::Black,
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
            Color::Good => self.cfg.colors.good,
            Color::Mediocre => self.cfg.colors.mediocre,
            Color::Bad => self.cfg.colors.bad,
        };
        write!(self.buf, "{}", console::Style::new().fg(color).apply_to(s)).unwrap()
    }

    fn write_sep(&mut self) {
        write!(
            self.buf,
            "{}",
            console::Style::new()
                .fg(self.cfg.separator_color)
                .apply_to(&self.cfg.separator)
        )
        .unwrap();
    }

    fn finish(&mut self) {
        println!("{}", self.buf);
    }
}
