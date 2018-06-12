use failure;
use output::Output;
pub mod battery;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WidgetKind {
    Volume,
    Mpd,
    Dynnet,
    DateTime,
    Battery,
}

pub trait Widget {
    fn run(&mut self, sink: &Output) -> Result<(), failure::Error>;
}

//pub use self::battery;
