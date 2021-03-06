use std::{fmt::Write, time::Duration};

use actix::prelude::{
    Actor, Addr, AsyncContext, Context, Handler, Message, SpawnHandle, SyncArbiter, SyncContext,
    System,
};
use log::*;
use notify_rust::Notification;

use super::status::Statusbar;
use crate::{
    config::{Config, Format},
    output::{output_from_format, Output},
};

fn format_error(err: &failure::Error) -> String {
    let mut ret = format!("{}\n", err);
    for cause in err.iter_causes() {
        writeln!(ret, "{}", cause).unwrap();
    }
    if ret.ends_with('\n') {
        ret.pop();
    }
    ret
}

pub struct Bar {
    bar: Statusbar,
    output: Box<dyn Output>,
    last_future_tick: SpawnHandle,
}

impl Actor for Bar {
    type Context = Context<Self>;
}

impl Bar {
    fn schedule_tick(&mut self, ctx: &mut Context<Self>) {
        self.last_future_tick = ctx.notify_later(Update, tick_duration(self.bar.update_interval()));
    }
}

fn tick_duration(interval: u32) -> Duration {
    Duration::from_millis(u64::from(interval))
}

impl Handler<Update> for Bar {
    type Result = ();
    fn handle(&mut self, _msg: Update, mut ctx: &mut Context<Self>) {
        self.schedule_tick(&mut ctx);
        self.bar.update(&mut *self.output);
    }
}

impl Handler<NewConfig> for Bar {
    type Result = ();
    fn handle(&mut self, NewConfig(cfg): NewConfig, mut ctx: &mut Context<Self>) {
        ctx.cancel_future(self.last_future_tick);
        let sep = cfg.general.separator.clone();
        let colors = cfg.colors.clone();
        match Statusbar::new(cfg, ctx.address()) {
            Ok(bar) => {
                self.bar = bar;
                self.output.set_sep(sep);
                self.output.set_colors(&colors);
                info!("Updated config");
                self.bar.update(&mut *self.output);
            }
            Err(e) => {
                ctx.address().do_send(ErrorLog(e));
            }
        }
        self.schedule_tick(&mut ctx);
    }
}

impl Handler<ErrorLog> for Bar {
    type Result = ();

    fn handle(&mut self, ErrorLog(msg): ErrorLog, _ctx: &mut Context<Self>) {
        error!("{}", msg);
        for cause in msg.iter_causes() {
            error!("Caused by: {}", cause)
        }

        if self.bar.desktop_notifications_enabled() {
            if let Err(e) = Notification::new()
                .summary("statusbar-rs error")
                // FIXME:
                .body(&format_error(&msg))
                .show()
            {
                warn!("{}", e);
            }
        }
    }
}

#[derive(Message)]
struct Update;

#[derive(Message)]
struct NewConfig(Config);

#[derive(Message)]
pub struct ErrorLog(pub failure::Error);

struct ConfigWatcher {
    tx: Addr<Bar>,
}

impl Actor for ConfigWatcher {
    type Context = SyncContext<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        use crate::config;
        use inotify::{Inotify, WatchMask};
        use std::thread;

        let mut inotify = Inotify::init().unwrap();

        let watch_config = |ino: &mut Inotify| {
            ino.add_watch(
                config::CONFIG_PATH.as_path(),
                WatchMask::CLOSE_WRITE | WatchMask::DELETE_SELF,
            )
        };

        let _ = watch_config(&mut inotify);

        // Ignore this unused, bug in nll
        let mut buf = [0u8; 4096];

        let tx = self.tx.clone();
        let mut on_event = move || -> Result<(), failure::Error> {
            let events = inotify.read_events_blocking(&mut buf)?;
            for event in events {
                if event.mask.contains(inotify::EventMask::DELETE_SELF)
                    && watch_config(&mut inotify).is_err()
                {
                    while watch_config(&mut inotify).is_err() {
                        thread::sleep(Duration::new(10, 0));
                    }
                }
            }

            let cfg = Config::load()?;

            tx.do_send(NewConfig(cfg));

            Ok(())
        };

        loop {
            if let Err(e) = on_event() {
                self.tx.do_send(ErrorLog(e));
            }
        }
    }
}

pub fn run(cfg: Config, output_format: Option<Format>) {
    let sys = System::new("bar");

    let format = output_format.unwrap_or(cfg.general.default_output_format);
    let mut output = output_from_format(cfg.general.separator.clone(), cfg.colors.clone(), format);
    output.init();

    let bar = Bar::create(move |ctx: &mut Context<Bar>| {
        let last = ctx.notify_later(Update, tick_duration(cfg.general.update_interval));

        let mut bar = Statusbar::new(cfg, ctx.address()).unwrap_or_else(|e| {
            ctx.address().do_send(ErrorLog(e));
            let general = crate::config::GeneralCfg::default();
            Statusbar::secure_default(ctx.address(), general)
        });
        bar.update(&mut *output);
        Bar {
            bar,
            last_future_tick: last,
            output,
        }
    });

    SyncArbiter::start(1, move || ConfigWatcher { tx: bar.clone() });
    sys.run();
}
