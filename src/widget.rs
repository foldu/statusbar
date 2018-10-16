pub mod battery;
pub mod cpu_usage;
pub mod datetime;
pub mod memory;
pub mod mpd;
pub mod net;
pub mod temp;
// FIXME: entire module assumes to be on linux
#[cfg(target_os = "linux")]
pub mod volume;

use serde_derive::{Deserialize, Serialize};

use crate::output::Output;

pub trait Widget {
    fn run(&mut self, _: &mut dyn Output) -> Result<(), failure::Error>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum WidgetKind {
    Battery(battery::Cfg),
    Datetime(datetime::Cfg),
    Mpd(mpd::Cfg),
    Volume(volume::Cfg),
    Net(net::Cfg),
    Memory(memory::Cfg),
    Temp(temp::Cfg),
    CpuUsage(cpu_usage::Cfg),
}

pub fn widget_from_kind(kind: WidgetKind) -> Result<Box<dyn Widget>, failure::Error> {
    use self::WidgetKind::*;
    Ok(match kind {
        Battery(cfg) => Box::new(battery::Widget::new(cfg)?),
        Datetime(cfg) => Box::new(datetime::Widget::new(cfg)),
        Mpd(cfg) => Box::new(mpd::Widget::new(cfg)?),
        Volume(cfg) => Box::new(volume::Widget::new(cfg)?),
        Net(cfg) => Box::new(net::Widget::new(cfg)?),
        Memory(cfg) => Box::new(memory::Widget::new(cfg)?),
        Temp(cfg) => Box::new(temp::Widget::new(cfg)?),
        CpuUsage(cfg) => Box::new(cpu_usage::Widget::new(cfg)?),
    })
}
