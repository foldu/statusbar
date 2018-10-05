pub mod awesome;
mod color;
pub mod i3;
pub mod terminal;

pub use self::color::ColorCfg;

use std::fmt;

use crate::config::Format;

pub trait Output {
    fn init(&mut self) {}
    fn start(&mut self) {}
    fn write(&mut self, _: fmt::Arguments);
    fn write_sep(&mut self) {}
    fn write_colored(&mut self, _: Color, _: fmt::Arguments);
    fn finish(&mut self) {}

    fn set_sep(&mut self, _: String) {}
    fn set_colors(&mut self, _: &ColorCfg);
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Good,
    Mediocre,
    Bad,
}

pub fn output_from_format(sep: String, colors: ColorCfg, fmt: Format) -> Box<dyn Output> {
    // FIXME: references to sep
    match fmt {
        Format::Awesome => Box::new(awesome::Output::new(&sep, colors)),
        Format::Terminal => Box::new(terminal::Output::new(&sep, colors)),
        Format::I3 => Box::new(i3::Output::new(colors)),
    }
}
