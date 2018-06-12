use output::Output;
use widget::Widget;

use failure;

pub struct BatteryWidget {}

impl BatteryWidget {
    pub fn new(_cfg: &Cfg) -> Self {
        BatteryWidget {}
    }
}

impl Widget for BatteryWidget {
    fn run(&mut self, _sink: &Output) -> Result<(), failure::Error> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Cfg {
    pub test: u32,
}
