pub mod battery;
pub mod datetime;
pub mod memory;
#[allow(dead_code)]
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
}

pub fn widget_from_kind(kind: WidgetKind) -> Box<dyn Widget> {
    use self::WidgetKind::*;
    match kind {
        Battery(cfg) => Box::new(battery::Widget::new(cfg)),
        Datetime(cfg) => Box::new(datetime::Widget::new(cfg)),
        Mpd(cfg) => Box::new(mpd::Widget::new(cfg)),
        Volume(cfg) => Box::new(volume::Widget::new(cfg)),
        Net(cfg) => Box::new(net::Widget::new(cfg)),
        Memory(cfg) => Box::new(memory::Widget::new(cfg)),
    }
}