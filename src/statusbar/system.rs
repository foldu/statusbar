use actix::prelude::*;
use std::time::Duration;

use super::statusbar::Statusbar;
use config::Config;
use widget::Widget;

struct Bar {
    bar: Statusbar,
    last_future_tick: SpawnHandle,
}

impl Actor for Bar {
    type Context = Context<Self>;
}

impl Bar {
    fn schedule_tick(&mut self, ctx: &mut Context<Self>) {
        self.last_future_tick = ctx.notify_later(Tick, tick_duration(self.bar.update_interval()));
    }
}

fn tick_duration(interval: u32) -> Duration {
    Duration::from_millis(interval as u64)
}

impl Handler<Tick> for Bar {
    type Result = ();
    fn handle(&mut self, _msg: Tick, mut ctx: &mut Context<Self>) {
        self.schedule_tick(&mut ctx);
        self.bar.render();
    }
}

impl Handler<NewConfig> for Bar {
    type Result = ();
    fn handle(&mut self, NewConfig(cfg): NewConfig, mut ctx: &mut Context<Self>) {
        ctx.cancel_future(self.last_future_tick);
        //self.cfg = cfg;
        info!("Updated config");
        self.schedule_tick(&mut ctx);
    }
}

#[derive(Message)]
struct Tick;

#[derive(Message)]
struct NewConfig(Config);

struct ConfigWatcher {
    tx: Addr<Syn, Bar>,
}

impl Actor for ConfigWatcher {
    type Context = SyncContext<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        use config;
        use inotify::{self, Inotify, WatchMask};
        use std::io;
        use std::thread;

        let mut inotify = Inotify::init().unwrap();

        let watch_config = |ino: &mut Inotify| {
            ino.add_watch(
                config::CONFIG_PATH.as_path(),
                WatchMask::CLOSE_WRITE | WatchMask::DELETE_SELF,
            )
        };

        let _ = watch_config(&mut inotify);

        let mut buf = [0u8; 4096];

        let mut on_event = move || -> Result<(), config::Error> {
            let events = inotify.read_events_blocking(&mut buf)?;
            for event in events {
                if event.mask.contains(inotify::EventMask::DELETE_SELF) {
                    if watch_config(&mut inotify).is_err() {
                        while watch_config(&mut inotify).is_err() {
                            thread::sleep(::std::time::Duration::new(10, 0));
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
    let bar: Addr<Syn, _> = Bar::create(|ctx: &mut Context<Bar>| {
        let last = ctx.notify_later(Tick, tick_duration(cfg.general.update_interval));
        // FIXME:
        let bar = Statusbar::new(cfg).unwrap();
        Bar {
            bar: bar,
            last_future_tick: last,
        }
    });

    SyncArbiter::start(1, move || ConfigWatcher { tx: bar.clone() });

    sys.run();
}
