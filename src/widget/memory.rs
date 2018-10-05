use serde_derive::{Deserialize, Serialize};

use crate::{
    formatter::{Format, FormatMap},
    output::Output,
};

pub struct Widget {
    cfg: Cfg,
    npages: u64,
    pagesize: u64,
    format_map: FormatMap,
}

impl Widget {
    pub fn new(cfg: Cfg) -> Self {
        let mut format_map = FormatMap::new();
        let npages = get_amount_of_pages();
        let pagesize = get_pagesize();
        format_map.insert("full", npages * pagesize);

        Self {
            format_map,
            cfg,
            npages,
            pagesize,
        }
    }
}

extern "C" {
    fn sysconf(name: i32) -> i64;
}

macro_rules! sysconf_function {
    ($id:ident, $sysconf_num:expr) => {
        fn $id() -> u64 {
            unsafe {
                let ret = sysconf($sysconf_num);
                if ret < 0 {
                    panic!("sysconf failed")
                }
                ret as u64
            }
        }
    };
}

#[cfg(target_os = "linux")]
mod sysconf {
    pub const SC_PHYS_PAGES: i32 = 85;
    pub const SC_AVPHYS_PAGES: i32 = 86;
    pub const SC_PAGESIZE: i32 = 30;
}

sysconf_function!(get_amount_of_pages, sysconf::SC_PHYS_PAGES);
sysconf_function!(get_available_pages, sysconf::SC_AVPHYS_PAGES);
sysconf_function!(get_pagesize, sysconf::SC_PAGESIZE);

impl super::Widget for Widget {
    fn run(&mut self, sink: &mut Output) -> Result<(), failure::Error> {
        let available_pages = get_available_pages();
        let used = self.npages - available_pages;
        // TODO: format these as bytes
        self.format_map.insert("used", used * self.pagesize);
        self.format_map.insert(
            "usedpercentage",
            available_pages as f64 / self.npages as f64 * 100.,
        );
        sink.write(format_args!(
            "{}",
            // FIXME: don't use fmt_owned
            self.cfg.format.fmt_owned(&self.format_map)?
        ));
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
    fn test_sysconf() {
        let _ = get_available_pages();
        let _ = get_amount_of_pages();
        let _ = get_pagesize();
    }
}
