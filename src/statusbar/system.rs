use std::time::Duration;

use actix::prelude::{
    Actor, Addr, AsyncContext, Context, Handler, Message, SpawnHandle, SyncArbiter, SyncContext,
    System,
};
use log::*;

use super::statusbar::Statusbar;
use crate::config::Config;

struct Bar {
    bar: Statusbar,
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
    Duration::from_millis(interval as u64)
}

impl Handler<Update> for Bar {
    type Result = ();
    fn handle(&mut self, _msg: Update, mut ctx: &mut Context<Self>) {
        self.schedule_tick(&mut ctx);
        self.bar.update();
    }
}

impl Handler<NewConfig> for Bar {
    type Result = ();
    fn handle(&mut self, NewConfig(cfg): NewConfig, mut ctx: &mut Context<Self>) {
        ctx.cancel_future(self.last_future_tick);
        self.bar = Statusbar::new(cfg).unwrap();
        info!("Updated config");
        self.bar.update();
        self.schedule_tick(&mut ctx);
    }
}

#[derive(Message)]
struct Update;

#[derive(Message)]
struct NewConfig(Config);

struct ConfigWatcher {
    tx: Addr<Bar>,
}

impl Actor for ConfigWatcher {
    type Context = SyncContext<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        use crate::config;
        use inotify::{self, Inotify, WatchMask};
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

        let mut on_event = move || -> Result<(), config::Error> {
            let events = inotify.read_events_blocking(&mut buf)?;
            for event in events {
                if event.mask.contains(inotify::EventMask::DELETE_SELF) {
                    if watch_config(&mut inotify).is_err() {
                        while watch_config(&mut inotify).is_err() {
                            thread::sleep(Duration::new(10, 0));
                        }
                    }
                }
            }

            let cfg = Config::load()?;

            self.tx.do_send(NewConfig(cfg));

            Ok(())
        };

        loop {
            if let Err(e) = on_event() {
                warn!("{}", e);
            }
        }
    }
}

pub fn run(cfg: Config) {
    let sys = System::new("bar");
    let bar = Bar::create(|ctx: &mut Context<Bar>| {
        let last = ctx.notify_later(Update, tick_duration(cfg.general.update_interval));
        // FIXME:
        let mut bar = Statusbar::new(cfg).unwrap();
        bar.update();
        Bar {
            bar,
            last_future_tick: last,
        }
    });

    SyncArbiter::start(1, move || ConfigWatcher { tx: bar.clone() });
    sys.run();
}
