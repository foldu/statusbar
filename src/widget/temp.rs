#[cfg(target_os = "linux")]
mod linux;

use formatter::{FormatMap, FormatString};
use serde_derive::{Deserialize, Serialize};

#[cfg(target_os = "linux")]
use self::linux::Sensor;
use crate::output::Output;

pub struct Widget {
    fmt_map: FormatMap,
    format: FormatString,
    unit: Unit,
    sensor: Sensor,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Result<Self, failure::Error> {
        let sensor = match cfg.dev {
            Device::FirstGpu => Sensor::first_gpu(),
            Device::FirstCpu => Sensor::first_cpu(),
            Device::Prefix(ref name) => Sensor::with_prefix(name),
        }?;

        Ok(Self {
            sensor,
            fmt_map: FormatMap::new(),
            unit: cfg.unit,
            format: FormatString::parse_with_allowed_keys(&cfg.format, &["temp"])?,
        })
    }
}

impl super::Widget for Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error> {
        let temp = self.sensor.get_temp()?;
        self.fmt_map.insert(
            "temp",
            match self.unit {
                Unit::Celsius => temp.0,
                Unit::Kelvin => Kelvin::from(temp).as_f64(),
                Unit::Fahrenheit => Fahrenheit::from(temp).as_f64(),
            },
        );

        sink.write(format_args!("{}", self.format.fmt(&self.fmt_map)?));

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Device {
    FirstGpu,
    FirstCpu,
    Prefix(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    Celsius,
    Kelvin,
    Fahrenheit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Cfg {
    unit: Unit,
    format: String,
    dev: Device,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            unit: Unit::Celsius,
            format: "cpu: {temp:.2}°C".to_owned(),
            dev: Device::FirstCpu,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Celsius(f64);

impl Celsius {
    fn new(n: f64) -> Self {
        Self { 0: n }
    }
}

macro_rules! make_temperature_unit {
    ($name:ident, $to_celsius:expr) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $name(f64);

        impl $name {
            #[allow(dead_code)]
            pub fn new(n: f64) -> Self {
                Self { 0: n }
            }

            pub fn as_f64(&self) -> f64 {
                self.0
            }
        }

        impl From<Celsius> for $name {
            fn from(celsius: Celsius) -> Self {
                Self {
                    0: $to_celsius(celsius.0),
                }
            }
        }
    };
}

const KELVIN_WATER_MELTING_POINT: f64 = 273.15;

make_temperature_unit!(Kelvin, |n| n + KELVIN_WATER_MELTING_POINT);

const ARBITRARY_BULLSHITE_FACTOR: f64 = 5. / 9.;
const ARBITRARY_BULLSHITE_SUMMAND: f64 = 32.;

make_temperature_unit!(Fahrenheit, |n| n * ARBITRARY_BULLSHITE_FACTOR
    + ARBITRARY_BULLSHITE_SUMMAND);
