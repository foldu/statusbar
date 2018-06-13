use failure;
use output::Output;
pub mod battery;
use config::Config;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum WidgetKind {
    Volume,
    Mpd,
    Dynnet,
    DateTime,
    Battery,
}

pub trait Widget {
    fn run(&mut self, sink: &mut Output) -> Result<(), failure::Error>;
}
