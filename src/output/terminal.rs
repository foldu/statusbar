use std::fmt::{self, Write};

use super::color::{ColorCfg, TerminalColors};
use crate::output::Color;

#[derive(Debug, Clone)]
pub struct Output {
    buf: String,
    cfg: Cfg,
}

#[derive(Debug, Clone)]
pub struct Cfg {
    separator: String,
    colors: TerminalColors,
}

impl Output {
    pub fn new(sep: &str, colors: ColorCfg) -> Self {
        Self {
            buf: String::new(),
            cfg: Cfg {
                colors: colors.terminal,
                separator: sep.to_owned(),
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
                .fg(self.cfg.colors.separator)
                .apply_to(&self.cfg.separator)
        )
        .unwrap();
    }

    fn finish(&mut self) {
        println!("{}", self.buf);
    }

    fn set_colors(&mut self, colors: &ColorCfg) {
        self.cfg.colors = colors.terminal.clone();
    }

    fn set_sep(&mut self, sep: String) {
        self.cfg.separator = sep;
    }
}
