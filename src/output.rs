pub mod awesome;
pub mod i3;
pub mod terminal;

use std::fmt;

use serde_derive::{Deserialize, Serialize};

use crate::{config::DefaultConfig, parse};

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
    I3(i3::Cfg),
}

pub fn output_from_kind(kind: OutputKind) -> Box<dyn Output> {
    use self::OutputKind::*;
    match kind {
        Awesome(cfg) => Box::new(awesome::Output::new(cfg)),
        Terminal(cfg) => Box::new(terminal::Output::new(cfg)),
        I3(cfg) => Box::new(i3::Output::new(cfg)),
    }
}

pub fn default_output(def: DefaultConfig) -> OutputKind {
    match def {
        DefaultConfig::Awesome => OutputKind::Awesome(awesome::Cfg::default()),
        DefaultConfig::Terminal => OutputKind::Terminal(terminal::Cfg::default()),
        DefaultConfig::I3 => OutputKind::I3(i3::Cfg::default()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GColors<C> {
    good: C,
    mediocre: C,
    bad: C,
}

#[derive(Debug, Clone)]
struct HexRgb(String);

impl std::str::FromStr for HexRgb {
    type Err = failure::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::hex_rgb(s).map(HexRgb)
    }
}

impl<'de> serde::de::Deserialize<'de> for HexRgb {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct HexRgbVisitor;
        impl<'de> serde::de::Visitor<'de> for HexRgbVisitor {
            type Value = HexRgb;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a hex rgb value like #001122")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                value
                    .parse::<HexRgb>()
                    .map_err(|e| E::custom(e.to_string()))
            }
        }

        deserializer.deserialize_str(HexRgbVisitor)
    }
}

impl serde::ser::Serialize for HexRgb {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}
