use std::fmt::{self, Write};

use serde_derive::{Deserialize, Serialize};

use super::color::{ColorCfg, GColors, HexRgb};
use crate::output::Color;

pub struct Output {
    buf: String,
    cfg: Cfg,
}

impl Output {
    pub fn new(colors: ColorCfg) -> Self {
        Self {
            buf: String::new(),
            cfg: Cfg { colors: colors.hex },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cfg {
    colors: GColors<HexRgb>,
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
            Color::Good => self.cfg.colors.good.as_ref(),
            Color::Mediocre => self.cfg.colors.mediocre.as_ref(),
            Color::Bad => self.cfg.colors.bad.as_ref(),
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
        if self.buf.ends_with(',') {
            self.buf.pop();
        }
        println!("{}],", self.buf);
    }
}
