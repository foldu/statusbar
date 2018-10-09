use std::fmt::Write;

use serde_derive::{Deserialize, Serialize};

use crate::{
    bytes::Bytes,
    formatter::{Format, FormatMap},
    output::Output,
};

pub struct Widget {
    cfg: Cfg,
    format_map: FormatMap,
    buf: String,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Self {
        let mut format_map = FormatMap::new();
        let mem_info = get_memory_info();
        format_map.insert("full", Bytes(mem_info.total).display_si().to_string());

        Self {
            format_map,
            cfg,
            buf: String::new(),
        }
    }
}

#[cfg(target_os = "linux")]
fn get_memory_info() -> MemoryInfo {
    use std::{fs::File, io::prelude::*, str};

    fn get_second_u64(s: &str) -> Option<u64> {
        s.split_whitespace().nth(1).and_then(|n| n.parse().ok())
    }

    let mut fh = File::open("/proc/meminfo").expect("Can't open /proc/meminfo");
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

        self.format_map.update_string_with("used", |s| {
            s.clear();
            write!(s, "{}", Bytes(mem_info.used).display_si()).unwrap();
        });

        self.format_map.insert(
            "usedpercentage",
            mem_info.used as f64 / mem_info.total as f64 * 100.,
        );

        self.buf.clear();
        self.cfg.format.fmt(&mut self.buf, &self.format_map)?;
        sink.write(format_args!("{}", &self.buf));
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cfg {
    format: Format,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memoryinfo_works() {
        let _ = get_memory_info();
    }
}
