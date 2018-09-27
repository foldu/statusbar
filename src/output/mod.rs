mod awesome;
pub use self::awesome::{AwesomeCfg, AwesomeOutput};

use std::fmt;

use serde_derive::{Deserialize, Serialize};

pub trait Output {
    fn init(&mut self) {}
    fn start(&mut self) {}
    fn write(&mut self, _: fmt::Arguments);
    fn write_sep(&mut self) {}
    fn write_colored(&mut self, _: Color, _: fmt::Arguments);
    fn finish(&mut self) {}
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Good,
    Mediocre,
    Bad,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OutputKind {
    Awesome(AwesomeCfg),
}

impl OutputKind {
    pub fn to_output(self) -> Box<dyn Output> {
        use self::OutputKind::*;
        match self {
            Awesome(cfg) => Box::new(AwesomeOutput::new(cfg)),
        }
    }
}

impl Default for OutputKind {
    fn default() -> Self {
        OutputKind::Awesome(AwesomeCfg::default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GColors<C> {
    good: C,
    mediocre: C,
    bad: C,
}
