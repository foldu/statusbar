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
    separator_color: console::Color,
    colors: TerminalColors,
}

impl Output {
    pub fn new(sep: &str, colors: &ColorCfg) -> Self {
        Self {
            buf: String::new(),
            cfg: Cfg {
                colors: colors.terminal.clone(),
                separator: sep.to_owned(),
                separator_color: colors.terminal_separator,
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
                .fg(self.cfg.separator_color)
                .apply_to(&self.cfg.separator)
        )
        .unwrap();
    }

    fn finish(&mut self) {
        println!("{}", self.buf);
    }
}
