use output::{Color, GColors, Output};
use parse;

#[derive(Debug, Clone)]
pub struct AwesomeOutput {
    buf: String,
    cfg: AwesomeCfg,
}

#[derive(Debug, Clone)]
pub struct HexRgb(String);

impl<'de> serde::de::Deserialize<'de> for HexRgb {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct HexRgbVisitor;
        impl<'de> serde::de::Visitor<'de> for HexRgbVisitor {
            type Value = HexRgb;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a hex rgb value like #001122")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                parse::hex_rgb(value)
                    .map_err(|e| E::custom(e.to_string()))
                    .map(HexRgb)
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwesomeCfg {
    separator: String,
    separator_color: HexRgb,
    colors: GColors<HexRgb>,
}

impl Default for AwesomeCfg {
    fn default() -> Self {
        Self {
            colors: GColors {
                good: HexRgb("#00FF00".into()),
                bad: HexRgb("#FF0000".into()),
                mediocre: HexRgb("#FFFF00".into()),
            },
            separator: " | ".into(),
            separator_color: HexRgb("#333333".into()),
        }
    }
}

impl AwesomeOutput {
    pub fn new(cfg: AwesomeCfg) -> Self {
        Self {
            buf: String::new(),
            cfg,
        }
    }
}

impl Output for AwesomeOutput {
    fn start(&mut self) {
        self.buf.clear();
    }

    fn write(&mut self, s: &str) {
        unimplemented!()
    }

    fn write_colored(&mut self, c: Color, s: &str) {
        unimplemented!()
    }

    fn write_sep(&mut self) {
        unimplemented!()
    }

    fn finish(&mut self) {
        println!("{}", self.buf);
    }
}
