use config::{Config, GeneralCfg, OutputCfg, OutputFormat};
use output::{AwesomeOutput, Output};
use widget::{battery::BatteryWidget, datetime::DateTimeWidget, Widget, WidgetKind};

use std::cell::RefCell;

pub struct Statusbar {
    widgets: Vec<Box<Widget>>,
    general_cfg: GeneralCfg,
    output: RefCell<Box<Output>>,
}

impl Statusbar {
    pub fn new(
        Config {
            general,
            formats: OutputCfg { awesome },
            battery,
            datetime,
        }: Config,
    ) -> Result<Self, failure::Error> {
        let ret = Self {
            widgets: general
                .order
                .iter()
                .map(move |k| match k {
                    WidgetKind::Battery => Ok(Box::new(BatteryWidget::new(battery)) as Box<Widget>),
                    WidgetKind::DateTime => {
                        Ok(Box::new(DateTimeWidget::new(datetime.clone())) as Box<Widget>)
                    }
                    _ => unimplemented!(),
                })
                .collect::<Result<Vec<_>, failure::Error>>()?,

            output: match general.output {
                OutputFormat::Awesome => RefCell::new(Box::new(AwesomeOutput::new(awesome))),
                _ => unimplemented!(),
            },

            general_cfg: general,
        };

        ret.output.borrow_mut().init();

        Ok(ret)
    }

    pub fn render(&mut self) {
        let mut out = self.output.borrow_mut();
        out.start();
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if i != 0 {
                out.write_sep();
            }
            if let Err(e) = widget.run(&mut **out) {
                warn!("{}", e);
            }
        }
        out.finish();
    }

    pub fn update_interval(&self) -> u32 {
        self.general_cfg.update_interval
    }
}
