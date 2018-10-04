pub mod awesome;
pub mod terminal;

use std::fmt;

use serde_derive::{Deserialize, Serialize};

use crate::config::DefaultConfig;

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
#[serde(tag = "type")]
pub enum OutputKind {
    Awesome(awesome::Cfg),
    Terminal(terminal::Cfg),
}

pub fn output_from_kind(kind: OutputKind) -> Box<dyn Output> {
    use self::OutputKind::*;
    match kind {
        Awesome(cfg) => Box::new(awesome::Output::new(cfg)),
        Terminal(cfg) => Box::new(terminal::Output::new(cfg)),
    }
}

pub fn default_output(def: DefaultConfig) -> OutputKind {
    match def {
        DefaultConfig::Awesome => OutputKind::Awesome(awesome::Cfg::default()),
        DefaultConfig::Terminal => OutputKind::Terminal(terminal::Cfg::default()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GColors<C> {
    good: C,
    mediocre: C,
    bad: C,
}
