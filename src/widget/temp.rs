#[cfg(target_os = "linux")]
mod linux;

use std::fmt;

use failure::format_err;
use serde_derive::{Deserialize, Serialize};

#[cfg(target_os = "linux")]
use self::linux::Sensor;
use crate::{
    formatter::{Format, FormatMap},
    output::Output,
};

pub struct Widget {
    format_map: FormatMap,
    buf: String,
    sensor: Sensor,
    cfg: Cfg,
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
            cfg,
            format_map: FormatMap::new(),
            buf: String::new(),
        })
    }
}

impl super::Widget for Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error> {
        let temp = self.sensor.get_temp()?;
        self.format_map.insert(
            "temp",
            match self.cfg.unit {
                Unit::Celsius => temp.0,
                Unit::Kelvin => Kelvin::from(temp).0,
            },
        );

        self.buf.clear();
        self.cfg.format.fmt(&mut self.buf, &self.format_map)?;
        sink.write(format_args!("{}", self.buf));

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Cfg {
    unit: Unit,
    format: Format,
    dev: Device,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            unit: Unit::Celsius,
            format: "cpu: {temp}C".parse().unwrap(),
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

#[derive(Debug, Copy, Clone)]
pub struct Kelvin(f64);

const KELVIN_WATER_MELTING_POINT: f64 = 273.15;

impl From<Celsius> for Kelvin {
    fn from(celsius: Celsius) -> Kelvin {
        Kelvin(celsius.0 + KELVIN_WATER_MELTING_POINT)
    }
}
