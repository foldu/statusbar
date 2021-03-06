use actix::prelude::Addr;

use super::system::{Bar, ErrorLog};
use crate::{
    config::{Config, GeneralCfg},
    output::Output,
    widget::{widget_from_kind, Widget},
};

pub struct Statusbar {
    widgets: Vec<Box<dyn Widget>>,
    general_cfg: GeneralCfg,
    controller: Addr<Bar>,
}

impl Statusbar {
    pub fn new(
        Config {
            widgets, general, ..
        }: Config,
        controller: Addr<Bar>,
    ) -> Result<Self, failure::Error> {
        let widgets = widgets
            .into_iter()
            .map(widget_from_kind)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            widgets,
            general_cfg: general,
            controller,
        })
    }

    pub fn update(&mut self, out: &mut dyn Output) {
        out.start();
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if i != 0 {
                out.write_sep();
            }
            if let Err(e) = widget.run(out) {
                self.controller.do_send(ErrorLog(e));
            }
        }
        out.finish();
    }

    pub fn update_interval(&self) -> u32 {
        self.general_cfg.update_interval
    }

    pub fn desktop_notifications_enabled(&self) -> bool {
        self.general_cfg.enable_desktop_notifications
    }

    pub fn secure_default(controller: Addr<Bar>, general_cfg: GeneralCfg) -> Self {
        use crate::widget::{datetime, net};
        Self {
            widgets: vec![
                Box::new(net::Widget::new(net::Cfg::default()).unwrap()),
                Box::new(datetime::Widget::new(datetime::Cfg::default())),
            ],
            general_cfg,
            controller,
        }
    }
}
