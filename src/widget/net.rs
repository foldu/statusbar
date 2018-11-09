#[cfg(test)]
mod tests;

#[cfg(target_os = "linux")]
mod linux;
mod unix;

use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
};

use delegate::*;
use failure::format_err;
use formatter::{FormatMap, FormatString};
use nix::sys::socket::{Ipv4Addr, Ipv6Addr};
use serde::{
    de::{Deserialize, Deserializer},
    ser::{SerializeSeq, Serializer},
};
use serde_derive::{Deserialize, Serialize};

use crate::{
    output::{Color, Output},
    widget,
};

pub struct Widget {
    cache: HashMap<String, IfInfo>,
    sock: unix::InetStreamSock,
    fmt_map: FormatMap,
    default_blacklist: InterfaceBlacklist,
    format_up: FormatString,
    format_down: FormatString,
    interface: Interface,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Result<Self, failure::Error> {
        Ok(Self {
            format_up: FormatString::parse_with_allowed_keys(
                &cfg.format_up,
                &["ipv4", "ipv6", "if"],
            )?,
            format_down: FormatString::parse_with_allowed_keys(&cfg.format_down, &[""])?,
            interface: cfg.interface,
            cache: HashMap::new(),
            sock: unix::InetStreamSock::new().expect("Can't create socket"),
            fmt_map: FormatMap::new(),
            default_blacklist: InterfaceBlacklist::new(),
        })
    }
}

fn best_running_if<'a>(cache: &'a HashMap<String, IfInfo>) -> Option<(&'a String, &'a IfInfo)> {
    let running = cache
        .iter()
        .filter(|(_, info)| info.is_running)
        .collect::<Vec<_>>();
    running
        .iter()
        .cloned()
        .find(|(_, info)| !info.type_.is_wireless())
        .or_else(|| running.into_iter().next())
}

impl widget::Widget for Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error> {
        let blacklist = match self.interface {
            Interface::Dynamic { ref blacklist } => blacklist,
            Interface::Device { .. } => &self.default_blacklist,
        };

        unix::update_ifs(&mut self.cache, &blacklist, self.sock);

        // FIXME: dude what
        let (color, is_up) = if let Some((if_, if_info)) = match self.interface {
            Interface::Dynamic { .. } => best_running_if(&self.cache),
            Interface::Device { ref name } => Some((
                name,
                self.cache
                    .get(name)
                    .ok_or_else(|| format_err!("Network interface {} doesn't exist", name))?,
            )),
        } {
            self.fmt_map.update_string_with("if", |s| s.clone_from(if_));
            self.fmt_map.update_string_with("ipv4", |s| {
                s.clear();
                if let Some(ipv4) = if_info.ipv4 {
                    write!(s, "{}", ipv4).unwrap();
                } else {
                    write!(s, "None").unwrap();
                }
            });
            self.fmt_map.update_string_with("ipv6", |s| {
                s.clear();
                if let Some(ipv6) = if_info.ipv6 {
                    write!(s, "{}", ipv6).unwrap();
                } else {
                    write!(s, "None").unwrap();
                }
            });

            if if_info.is_running && (if_info.ipv4.is_some() || if_info.ipv6.is_some()) {
                (Color::Good, true)
            } else if if_info.is_running {
                (Color::Mediocre, true)
            } else {
                (Color::Bad, false)
            }
        } else {
            (Color::Bad, false)
        };

        let format = if is_up {
            &self.format_up
        } else {
            &self.format_down
        };

        sink.write_colored(color, format_args!("{}", format.fmt(&self.fmt_map)?));

        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
enum IfType {
    Ethernet,
    Wireless,
}

impl IfType {
    fn is_wireless(self) -> bool {
        if let IfType::Wireless = self {
            true
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IfInfo {
    ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
    is_running: bool,
    type_: IfType,
}

#[derive(Clone, Debug)]
pub struct InterfaceBlacklist(HashSet<String>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Interface {
    Dynamic {
        #[serde(deserialize_with = "deserialize_blacklist")]
        #[serde(serialize_with = "serialize_blacklist")]
        blacklist: InterfaceBlacklist,
    },
    Device {
        name: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cfg {
    format_up: String,
    format_down: String,
    interface: Interface,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            interface: Interface::Dynamic {
                blacklist: InterfaceBlacklist::new(),
            },
            format_up: "{if}: {ipv4}".to_owned(),
            format_down: "net: no".to_owned(),
        }
    }
}

impl InterfaceBlacklist {
    #[inline]
    pub fn new() -> Self {
        let mut ret = HashSet::with_capacity(1);
        ret.insert("lo".to_owned());
        Self { 0: ret }
    }

    delegate! {
        target self.0 {
            pub fn contains(&self, if_:&str) -> bool;
            pub fn len(&self) -> usize;
        }
    }
}

fn deserialize_blacklist<'de, D>(de: D) -> Result<InterfaceBlacklist, D::Error>
where
    D: Deserializer<'de>,
{
    let mut ret = HashSet::deserialize(de)?;
    ret.insert("lo".to_owned());
    Ok(InterfaceBlacklist(ret))
}

fn serialize_blacklist<S>(blacklist: &InterfaceBlacklist, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(blacklist.len()))?;
    for elem in blacklist.0.iter().filter(|elem| elem.as_str() != "lo") {
        seq.serialize_element(elem)?;
    }

    seq.end()
}
