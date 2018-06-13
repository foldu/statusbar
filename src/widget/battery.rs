use output::Output;
use widget::Widget;

use failure;

pub struct BatteryWidget {}

impl BatteryWidget {
    pub fn new(_cfg: Cfg) -> Self {
        BatteryWidget {}
    }
}

impl Widget for BatteryWidget {
    fn run(&mut self, sink: &mut Output) -> Result<(), failure::Error> {
        sink.write(format_args!("test"));
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, Copy)]
pub struct Cfg {
    pub test: u32,
}
