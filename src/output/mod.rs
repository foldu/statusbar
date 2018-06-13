mod awesome;
pub use self::awesome::{AwesomeCfg, AwesomeOutput};
use notify_rust::Notification;

use std::fmt;

pub trait Output {
    fn init(&mut self) {}
    fn start(&mut self) {}
    fn write(&mut self, fmt::Arguments);
    fn write_sep(&mut self) {}
    fn write_colored(&mut self, Color, fmt::Arguments);
    fn finish(&mut self) {}
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Good,
    Mediocre,
    Bad,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GColors<C> {
    good: C,
    mediocre: C,
    bad: C,
}
