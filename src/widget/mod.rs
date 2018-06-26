pub mod battery;
pub mod datetime;

use config::Config;
use output::Output;
//
pub trait Widget {
    fn run(&mut self, sink: &mut dyn Output) -> Result<(), failure::Error>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WidgetKind {
    Battery(battery::Cfg),
    Datetime(datetime::Cfg),
}

impl WidgetKind {
    pub fn to_widget(self) -> Box<dyn Widget> {
        use self::WidgetKind::*;
        match self {
            Battery(cfg) => Box::new(battery::BatteryWidget::new(cfg)),
            Datetime(cfg) => Box::new(datetime::DatetimeWidget::new(cfg)),
        }
    }
}
