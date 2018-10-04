use std::cell::RefCell;

use actix::prelude::Addr;

use super::system::{Bar, ErrorLog};
use crate::{
    config::{Config, Format, GeneralCfg},
    output::{output_from_format, Output},
    widget::{widget_from_kind, Widget},
};

pub struct Statusbar {
    widgets: Vec<Box<dyn Widget>>,
    general_cfg: GeneralCfg,
    output: RefCell<Box<dyn Output>>,
    controller: Addr<Bar>,
}

impl Statusbar {
    pub fn new(
        Config {
            widgets,
            general,
            colors,
        }: Config,
        controller: Addr<Bar>,
        format: Format,
    ) -> Self {
        let ret = Self {
            widgets: widgets.into_iter().map(widget_from_kind).collect(),
            output: RefCell::new(output_from_format(&general.separator, colors, format)),
            general_cfg: general,
            controller,
        };

        ret.output.borrow_mut().init();

        ret
    }

    pub fn update(&mut self) {
        let mut out = self.output.borrow_mut();
        out.start();
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if i != 0 {
                out.write_sep();
            }
            if let Err(e) = widget.run(&mut **out) {
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
}
