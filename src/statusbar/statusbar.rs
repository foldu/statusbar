//use std::cell::RefCell;
//
//use config::{Config, GeneralCfg, OutputFormat};
//use output::{AwesomeOutput, Output};
//use widget::{widget_from_kind, Widget};
//
//pub struct Statusbar {
//    widgets: Vec<Box<dyn Widget>>,
//    general_cfg: GeneralCfg,
//    output: RefCell<Box<dyn Output>>,
//}
//
//impl Statusbar {
//    pub fn new(cfg: Config) -> Result<Self, failure::Error> {
//        let ret = Self {
//            widgets: cfg
//                .general
//                .order
//                .iter()
//                .map(|&kind| widget_from_kind(&cfg, kind))
//                .collect::<Result<Vec<_>, failure::Error>>()?,
//
//            output: match cfg.general.output {
//                OutputFormat::Awesome => {
//                    RefCell::new(Box::new(AwesomeOutput::new(cfg.formats.awesome)))
//                }
//                _ => unimplemented!(),
//            },
//
//            general_cfg: cfg.general,
//        };
//
//        ret.output.borrow_mut().init();
//
//        Ok(ret)
//    }
//
//    pub fn render(&mut self) {
//        let mut out = self.output.borrow_mut();
//        out.start();
//        for (i, widget) in self.widgets.iter_mut().enumerate() {
//            if i != 0 {
//                out.write_sep();
//            }
//            if let Err(e) = widget.run(&mut **out) {
//                warn!("{}", e);
//            }
//        }
//        out.finish();
//    }
//
//    pub fn update_interval(&self) -> u32 {
//        self.general_cfg.update_interval
//    }
//}
