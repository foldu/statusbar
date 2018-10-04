use serde_derive::*;

use crate::parse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GColors<C> {
    pub good: C,
    pub mediocre: C,
    pub bad: C,
}

#[derive(Debug, Clone)]
pub struct HexRgb(String);

impl AsRef<str> for HexRgb {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

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
pub struct TerminalColors {
    #[serde(with = "ColorDef")]
    pub good: console::Color,
    #[serde(with = "ColorDef")]
    pub mediocre: console::Color,
    #[serde(with = "ColorDef")]
    pub bad: console::Color,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorCfg {
    pub hex_separator: HexRgb,
    #[serde(with = "ColorDef")]
    pub terminal_separator: console::Color,
    pub hex: GColors<HexRgb>,
    pub terminal: TerminalColors,
}

impl Default for ColorCfg {
    fn default() -> Self {
        Self {
            hex: GColors {
                good: "#00FF00".parse().unwrap(),
                bad: "#FF0000".parse().unwrap(),
                mediocre: "#FFFF00".parse().unwrap(),
            },

            hex_separator: "#333333".parse().unwrap(),

            terminal: TerminalColors {
                good: console::Color::Green,
                bad: console::Color::Red,
                mediocre: console::Color::Yellow,
            },

            terminal_separator: console::Color::Black,
        }
    }
}
