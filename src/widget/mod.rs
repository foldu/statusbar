pub mod battery;
pub mod datetime;

use config::Config;
use output::Output;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum WidgetKind {
    Volume,
    Mpd,
    Dynnet,
    DateTime,
    Battery,
}

pub trait Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error>;
}

pub fn widget_from_kind(cfg: &Config, kind: WidgetKind) -> Result<Box<dyn Widget>, failure::Error> {
    match kind {
        WidgetKind::DateTime => Ok(Box::new(datetime::DateTimeWidget::new(
            cfg.datetime.clone(),
        ))),
        _ => unimplemented!(),
    }
}
