use failure;
use output::Output;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
