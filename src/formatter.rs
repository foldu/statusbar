use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

use failure::Fail;

#[derive(Debug, Clone)]
pub enum Op {
    PutString(String),
    PutMap(String),
    PutMapTrunc(String, usize),
}

#[derive(Debug, Clone)]
pub struct FormatMap(HashMap<String, MapCont>);

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Key \"{}\" not in map", _0)]
    NotInMap(String),
    #[fail(display = "{}", _0)]
    Fmt(#[cause] fmt::Error),
    #[fail(display = "{}", _0)]
    Parse(#[cause] ParseError),
}

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "Empty ident")]
    EmptyIdent(usize),
    #[fail(display = "Truncation missing")]
    MissingTrunc(usize),
    #[fail(display = "Truncation specifier contains non digit {}", _0)]
    TruncNotDigit(usize, char),
    #[fail(display = "Truncation specifier is too big and overflowed")]
    TruncOverflow(usize),
    #[fail(display = "Invalid ident: {}", _0)]
    InvalidIdent(usize, String),
}

impl ParseError {
    pub fn pretty(&self, _s: &str) -> String {
        unimplemented!()
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::Fmt(e)
    }
}

impl FormatMap {
    pub fn new() -> Self {
        Self { 0: HashMap::new() }
    }
    fn get(&self, k: &str) -> Option<&MapCont> {
        self.0.get(k)
    }

    pub fn insert<C>(&mut self, k: &str, a: C)
    where
        C: Into<MapCont>,
    {
        let cont = a.into();
        if let Some(cell) = self.0.get_mut(k) {
            *cell = cont;
        } else {
            self.0.insert(k.to_owned(), cont);
        }
    }
}

#[derive(Debug, Clone)]
pub enum MapCont {
    Unsigned(u64),
    Str(String),
    Float(f64),
}

impl From<u64> for MapCont {
    fn from(n: u64) -> Self {
        MapCont::Unsigned(n)
    }
}

impl From<f64> for MapCont {
    fn from(n: f64) -> Self {
        MapCont::Float(n)
    }
}

impl From<String> for MapCont {
    fn from(s: String) -> Self {
        MapCont::Str(s)
    }
}

#[derive(Debug, Clone)]
pub struct Format(Vec<Op>);

impl Format {
    pub fn fmt_no_lookup(&self, ret: &mut String) -> Result<(), Error> {
        for op in self.0.iter() {
            match op {
                Op::PutString(ref s) => write!(ret, "{}", s)?,
                Op::PutMap(ref s) => write!(ret, "{{{}}}", s)?,
                Op::PutMapTrunc(ref s, n) => write!(ret, "{{{}:{}}}", s, n)?,
            }
        }
        Ok(())
    }

    pub fn fmt(&self, ret: &mut String, fmt_map: &FormatMap) -> Result<(), Error> {
        let put_from_fmt_map = |ret: &mut String, k: &str| {
            if let Some(ref cont) = fmt_map.get(k) {
                match cont {
                    MapCont::Unsigned(n) => write!(ret, "{}", n)?,
                    MapCont::Float(n) => write!(ret, "{}", n)?,
                    MapCont::Str(s) => ret.push_str(s),
                };
            } else {
                return Err(Error::NotInMap(k.into()));
            }
            Ok(())
        };

        for op in self.0.iter() {
            match op {
                Op::PutString(ref s) => ret.push_str(s),
                Op::PutMap(ref id) => put_from_fmt_map(ret, id)?,
                Op::PutMapTrunc(ref id, n) => {
                    let old_len = ret.len();
                    put_from_fmt_map(ret, id)?;
                    ret.truncate(old_len + n);
                }
            }
        }
        Ok(())
    }

    pub fn fmt_owned(&self, fmt_map: &FormatMap) -> Result<String, Error> {
        let mut s = String::new();
        self.fmt(&mut s, fmt_map)?;
        Ok(s)
    }

    pub fn parse(s: &str) -> Result<Self, ParseError> {
        enum State {
            GotCurlyOpen,
            InCurly,
            ReadTrunc,
            Normal,
        }

        let mut ret = vec![];
        let mut state = State::Normal;

        let mut str_buf = String::new();
        let mut trunc_len = 0;

        for (i, c) in s.chars().enumerate() {
            state = match state {
                State::Normal => {
                    if c == '{' {
                        State::GotCurlyOpen
                    } else {
                        str_buf.push(c);
                        State::Normal
                    }
                }
                State::GotCurlyOpen => {
                    if c == '{' {
                        str_buf.push('{');
                        State::Normal
                    } else if c == '}' {
                        return Err(ParseError::EmptyIdent(i));
                    } else {
                        ret.push(Op::PutString(str_buf.clone()));
                        str_buf.clear();
                        str_buf.push(c);
                        if !c.is_alphabetic() {
                            return Err(ParseError::InvalidIdent(i, str_buf));
                        }
                        State::InCurly
                    }
                }
                State::InCurly => {
                    if c == '}' {
                        ret.push(Op::PutMap(str_buf.clone()));
                        str_buf.clear();
                        State::Normal
                    } else if c == ':' {
                        if str_buf.is_empty() {
                            return Err(ParseError::EmptyIdent(i));
                        } else {
                            State::ReadTrunc
                        }
                    } else if c.is_alphabetic() {
                        str_buf.push(c);
                        State::InCurly
                    } else {
                        str_buf.push(c);
                        return Err(ParseError::InvalidIdent(i, str_buf));
                    }
                }
                State::ReadTrunc => {
                    if c == '}' {
                        if trunc_len != 0 {
                            ret.push(Op::PutMapTrunc(str_buf.clone(), trunc_len));
                            trunc_len = 0;
                        } else {
                            return Err(ParseError::MissingTrunc(i));
                        }
                        State::Normal
                    } else if c.is_ascii_digit() {
                        let n = (c as u8 - 48) as usize;
                        trunc_len = if let Some(len) =
                            trunc_len.checked_mul(10).and_then(|len| len.checked_add(n))
                        {
                            len
                        } else {
                            return Err(ParseError::TruncOverflow(i));
                        };

                        State::ReadTrunc
                    } else {
                        return Err(ParseError::TruncNotDigit(i, c));
                    }
                }
            }
        }

        Ok(Self { 0: ret })
    }
}
