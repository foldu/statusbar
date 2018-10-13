use formatter::{FormatMap, FormatString};
use serde_derive::{Deserialize, Serialize};

use crate::output::Output;

pub struct Widget {
    format: FormatString,
    fmt_map: FormatMap,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Result<Self, failure::Error> {
        let mut fmt_map = FormatMap::new();
        let mem_info = get_memory_info();
        fmt_map.insert("full", mem_info.total as f64);

        Ok(Self {
            fmt_map,
            format: FormatString::parse_with_allowed_keys(
                &cfg.format,
                &["full", "used", "percent_used"],
            )?,
        })
    }
}

#[cfg(target_os = "linux")]
fn get_memory_info() -> MemoryInfo {
    use std::{fs::File, io::prelude::*, str};

    fn get_second_u64(s: &str) -> Option<u64> {
        s.split_whitespace().nth(1).and_then(|n| n.parse().ok())
    }

    let mut fh = File::open("/proc/meminfo").expect("Can't open /proc/meminfo");
    // just assume all interesting parts of /proc/meminfo fit in 8192 bytes
    let mut buf = [0; 8192];
    let nbytes = fh.read(&mut buf).unwrap();

    let mut mem_total: Option<u64> = None;
    let mut mem_free: Option<u64> = None;
    let mut buffers: Option<u64> = None;
    let mut cached: Option<u64> = None;

    for ln in str::from_utf8(&buf[..nbytes])
        .expect("ascii only /proc/meminfo isn't valid utf-8")
        .split('\n')
    {
        if ln.starts_with("MemTotal") {
            mem_total = get_second_u64(ln);
        } else if ln.starts_with("MemFree") {
            mem_free = get_second_u64(ln);
        } else if ln.starts_with("Buffers") {
            buffers = get_second_u64(ln);
        } else if ln.starts_with("Cached") {
            cached = get_second_u64(ln);
        }
    }

    (|| {
        let mem_total = mem_total? * 1000;
        let mem_free = mem_free? * 1000;
        let buffers = buffers? * 1000;
        let cached = cached? * 1000;
        Some(MemoryInfo {
            total: mem_total,
            used: mem_total - mem_free - cached - buffers,
        })
    })()
    .expect("Something went wrong")
}

#[derive(Clone, Copy, Debug)]
struct MemoryInfo {
    total: u64,
    used: u64,
}

impl super::Widget for Widget {
    fn run(&mut self, sink: &mut Output) -> Result<(), failure::Error> {
        let mem_info = get_memory_info();

        self.fmt_map.insert("used", mem_info.used as f64);

        self.fmt_map.insert(
            "percent_used",
            mem_info.used as f64 / mem_info.total as f64 * 100.,
        );

        sink.write(format_args!("{}", self.format.fmt(&self.fmt_map)?));
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cfg {
    format: String,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            format: "{used:S.2}/{full:S.2} {percent_used:.2}%".parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memoryinfo_works() {
        let _ = get_memory_info();
    }
}
