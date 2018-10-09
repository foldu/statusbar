#[cfg(target_os = "linux")]
mod linux;

use std::fmt;

use failure::format_err;

#[derive(Debug, Copy, Clone)]
struct Celsius(f64);

impl Celsius {
    fn new(n: f64) -> Self {
        Self { 0: n }
    }
}

#[derive(Debug, Copy, Clone)]
struct Kelvin(f64);

const KELVIN_WATER_MELTING_POINT: f64 = 273.15;

impl From<Celsius> for Kelvin {
    fn from(celsius: Celsius) -> Kelvin {
        Kelvin(celsius.0 + KELVIN_WATER_MELTING_POINT)
    }
}
