pub mod error;
mod parser;
#[cfg(test)]
mod tests;

use std::{collections::HashMap, fmt, str, time::Duration};

use noisy_float::prelude::*;
use num_traits::cast::ToPrimitive;
use pest::iterators::Pairs;

pub use crate::error::Error;
use crate::parser::Rule;

#[derive(Debug, Copy, Clone)]
pub enum Unit {
    Si,
    Bin,
}

enum Op {
    Str(String),
    FromMap { key: String, fmt_opt: FormatOptions },
}

pub struct FormatOptions {
    trunc: Option<usize>,
    unit: Option<Unit>,
    significant_digits: Option<u8>,
}

pub struct FormatString(Vec<Op>);

fn eval_format<F>(pairs: Pairs<Rule>, is_valid_key: F) -> Result<Op, Error>
where
    F: Fn(&str) -> Result<(), Error>,
{
    let mut key = None;
    let mut trunc: Option<usize> = None;
    let mut unit: Option<Unit> = None;
    let mut significant_digits: Option<u8> = None;

    for pair in pairs {
        match pair.as_rule() {
            Rule::ident => {
                let ident = pair.as_str();
                is_valid_key(&ident)?;
                key = Some(ident.to_owned())
            }
            Rule::format_spec => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::precision => {
                            significant_digits = Some(pair.as_str().parse().unwrap());
                        }
                        Rule::unit => {
                            unit = Some(match pair.as_str() {
                                "B" => Unit::Bin,
                                "S" => Unit::Si,
                                _ => unreachable!(),
                            });
                        }
                        Rule::trunc => {
                            trunc = Some(pair.as_str().parse().unwrap());
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(Op::FromMap {
        key: key.unwrap(),
        fmt_opt: FormatOptions {
            unit,
            trunc,
            significant_digits,
        },
    })
}

pub struct DelayedFormat<'a> {
    format: &'a FormatString,
    map: &'a FormatMap,
}

impl std::fmt::Display for DelayedFormat<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for op in &self.format.0 {
            match op {
                Op::Str(ref s) => {
                    fmt.write_str(s)?;
                }
                Op::FromMap { key, fmt_opt } => {
                    let cont = self.map.get(key).unwrap();
                    cont.format_with(fmt_opt, fmt)?;
                }
            }
        }
        Ok(())
    }
}

impl FormatString {
    pub fn fmt<'a>(&'a self, map: &'a FormatMap) -> Result<DelayedFormat<'a>, Error> {
        // maybe put all needed keys in a hashset?
        if let Some(key) = self
            .0
            .iter()
            .filter_map(|op| {
                if let Op::FromMap { ref key, .. } = op {
                    Some(key)
                } else {
                    None
                }
            })
            .find(|key| !map.0.contains_key(key.as_str()))
        {
            Err(Error::KeyNotInMap(key.to_owned()))
        } else {
            Ok(DelayedFormat { format: &self, map })
        }
    }

    fn parse_with_key_validator<F>(s: &str, is_valid_key: F) -> Result<Self, Error>
    where
        F: Fn(&str) -> Result<(), Error> + Copy,
    {
        let mut ret = vec![];
        let parsed = parser::parse(s)?;

        for pair in parsed {
            match pair.as_rule() {
                Rule::maybe_format => {
                    let inner = pair.into_inner().next().unwrap();
                    match inner.as_rule() {
                        // FIXME: merge with text
                        Rule::open_curly => ret.push(Op::Str("{".to_owned())),
                        Rule::close_curly => ret.push(Op::Str("}".to_owned())),
                        Rule::format => ret.push(eval_format(inner.into_inner(), is_valid_key)?),
                        _ => unreachable!(),
                    }
                }
                Rule::text => {
                    ret.push(Op::Str(pair.as_str().to_owned()));
                }
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }

        Ok(Self { 0: ret })
    }

    pub fn parse_with_allowed_keys<S>(s: &str, allowed: &[S]) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        Self::parse_with_key_validator(s, |key| {
            if allowed
                .iter()
                .any(|allowed_key| key == allowed_key.as_ref())
            {
                Ok(())
            } else {
                Err(Error::InvalidKey {
                    key: key.to_owned(),
                    allowed: allowed.iter().map(|s| s.as_ref().to_owned()).collect(),
                })
            }
        })
    }

    pub fn parse(s: &str) -> Result<Self, Error> {
        Self::parse_with_key_validator(s, |_| Ok(()))
    }
}

pub trait Formatable {
    fn format_with(&self, opt: &FormatOptions, writer: &mut fmt::Formatter) -> fmt::Result;
}

impl Formatable for dyn fmt::Display {
    fn format_with(&self, _opt: &FormatOptions, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(fmt)
    }
}

impl Formatable for MapCont {
    fn format_with(&self, opt: &FormatOptions, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MapCont::Str(ref s) => {
                // find char boundary at trunc len, otherwise just format the entire string
                let bytes_to_write = if let Some(trunc) = opt.trunc {
                    s.char_indices()
                        .enumerate()
                        .find_map(
                            |(i, (index, _))| {
                                if i == trunc {
                                    Some(index)
                                } else {
                                    None
                                }
                            },
                        )
                        .unwrap_or_else(|| s.len())
                } else {
                    s.len()
                };

                // safe because bytes_to_write is always on a char boundary
                fmt.write_str(unsafe {
                    str::from_utf8_unchecked(&s.as_bytes()[..bytes_to_write])
                })?;
                Ok(())
            }

            MapCont::Number(n) => {
                let significant_digits = opt.significant_digits.unwrap_or(0);

                const SI_LOOKUP: [&str; 6] = ["", "k", "M", "G", "T", "P"];
                const BIN_LOOKUP: [&str; 6] = ["", "ki", "Mi", "Gi", "Ti", "Pi"];

                if let Some(unit) = opt.unit {
                    let ((out_n, index), table) = match unit {
                        Unit::Si => (unitize_si(*n), &SI_LOOKUP),
                        Unit::Bin => (unitize_bin(*n), &BIN_LOOKUP),
                    };
                    write_float(fmt, out_n, significant_digits)?;
                    fmt.write_str(table[index])?;
                    Ok(())
                } else {
                    write_float(fmt, *n, significant_digits)
                }
            }

            MapCont::Duration(duration) => {
                let minutes = duration.as_secs() / 60;
                let seconds = duration.as_secs() - (minutes * 60);
                write!(fmt, "{:02}:{:02}", minutes, seconds)
            }
        }
    }
}

fn write_float(fmt: &mut fmt::Formatter<'_>, n: R64, significant_digits: u8) -> fmt::Result {
    let mut rbuf = ryu::Buffer::new();
    let s = rbuf.format(n.to_f64().unwrap());
    // should be no problem because it's a R64, max(1.0) is called on it ergo log10 should always
    // work and floor always works too
    let digits_before_dot = n
        .abs()
        .max(R64::new(1.0))
        .log10()
        .floor()
        .to_usize()
        .unwrap()
        + 1;
    let bytes_to_write = if significant_digits == 0 {
        digits_before_dot
    } else {
        digits_before_dot + 1 + significant_digits as usize
    } + if n < 0.0 { 1 } else { 0 };

    // safe because the content of rbuf is always valid ascii
    let to_write =
        unsafe { str::from_utf8_unchecked(&s.as_bytes()[..bytes_to_write.min(s.len())]) };
    fmt.write_str(to_write)?;

    // pad end with zeroes if formatted float contains too few
    if bytes_to_write > s.len() {
        let zero = "0";
        for _ in 0..(bytes_to_write - s.len()) {
            fmt.write_str(zero)?;
        }
    }
    Ok(())
}

// FIXME: terrible name
// FIXME: terrible implementation
macro_rules! unitize {
    ($limit:expr, $ident:ident) => {
        fn $ident(mut n: R64) -> (R64, usize) {
            let mut index = 0;
            while n >= $limit {
                n /= $limit;
                index += 1;
            }
            (n, index)
        }
    };
}
// FIXME: terrible names
unitize!(1024., unitize_bin);
unitize!(1000., unitize_si);

pub enum MapCont {
    Number(R64),
    Str(String),
    Duration(Duration),
}

impl From<String> for MapCont {
    fn from(s: String) -> Self {
        MapCont::Str(s)
    }
}

impl From<R64> for MapCont {
    fn from(n: R64) -> Self {
        MapCont::Number(n)
    }
}

impl From<f64> for MapCont {
    fn from(n: f64) -> Self {
        MapCont::Number(R64::new(n))
    }
}

impl From<Duration> for MapCont {
    fn from(duration: Duration) -> Self {
        MapCont::Duration(duration)
    }
}

#[derive(Default)]
pub struct FormatMap(HashMap<String, MapCont>);

impl FormatMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<C>(&mut self, key: &str, cont: C)
    where
        C: Into<MapCont>,
    {
        let to_insert = cont.into();
        if let Some(cell) = self.0.get_mut(key) {
            *cell = to_insert;
        } else {
            self.0.insert(key.to_owned(), to_insert);
        }
    }

    pub fn update_string_with<F>(&mut self, key: &str, f: F)
    where
        F: FnOnce(&mut String),
    {
        if let Some(MapCont::Str(s)) = self.0.get_mut(key) {
            s.clear();
            f(s);
        } else {
            let mut cont = String::new();
            f(&mut cont);
            self.0.insert(key.to_owned(), cont.into());
        }
    }

    fn get(&self, key: &str) -> Option<&MapCont> {
        self.0.get(key)
    }
}
