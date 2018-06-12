mod awesome;
pub use self::awesome::{AwesomeCfg, AwesomeOutput};
use notify_rust::Notification;

// FIXME: interface sucks
pub trait Output {
    fn warn(&mut self, e: &failure::Error) {
        if let Err(e) = Notification::new()
            .summary("statusbar-rs")
            .body(&e.to_string())
            .show()
        {
            eprintln!("Error erroring: {}", e);
        }
    }
    fn init(&mut self) {}
    fn start(&mut self) {}
    fn write(&mut self, &str);
    fn write_sep(&mut self) {}
    fn write_colored(&mut self, Color, &str);
    fn finish(&mut self) {}
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Good,
    Mediocre,
    Bad,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GColors<C> {
    good: C,
    mediocre: C,
    bad: C,
}
