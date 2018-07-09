#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! termion = "*"
//! ```
extern crate termion;

use std::ffi::OsStr;
use std::io::{self, prelude::*};
use std::process::{exit, Command, Output};
use std::str;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use termion::color;

fn read_cmd_output<S, I>(cmd: &str, args: I) -> io::Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(cmd).args(args).output()
}

struct Spinner {
    hndl: thread::JoinHandle<()>,
    tx: Sender<()>,
}

const SPIN_ARR: &[u8; 32] = b"/-\\|/-\\|/-\\|/-\\|\\-/|\\-/|\\-/|\\-/|";

impl Spinner {
    fn spawn() -> Self {
        let (tx, rx) = channel();
        let hndl = thread::spawn(move || {
            let mut i = 0;
            loop {
                print!("{}", SPIN_ARR[i] as char);
                flush_stdout();
                if rx.try_recv().is_ok() {
                    break;
                }
                thread::sleep_ms(150);
                i += 1;
                i %= SPIN_ARR.len();
                print!("{}", 8 as char);
            }
        });

        Self { hndl, tx }
    }

    fn stop(self) {
        self.tx.send(()).unwrap();
        self.hndl.join().unwrap();
    }
}

fn flush_stdout() {
    io::stdout().flush().unwrap();
}

fn print_ok(s: &str) {
    println!(
        "{}v {}{}",
        color::Fg(color::Green),
        s,
        color::Fg(color::Reset)
    );
}

fn print_fail(s: &str) {
    println!(
        "{}x {}{}",
        color::Fg(color::Red),
        s,
        color::Fg(color::Reset)
    );
}

fn clear_line() {
    print!("{}\r", termion::clear::CurrentLine);
}

fn format() -> bool {
    print!("Running cargo fmt");
    flush_stdout();

    if read_cmd_output("cargo", &["fmt"]).is_ok() {
        clear_line();
        print_ok("Code formatted");
        true
    } else {
        clear_line();
        print_fail("rustfmt not installed");
        false
    }
}

fn run_tests() -> bool {
    print!("Running tests, please wait warmly ");
    flush_stdout();
    let spin = Spinner::spawn();
    let out = read_cmd_output("cargo", &["test"]).unwrap();
    spin.stop();
    clear_line();
    if out.status.success() {
        print_ok("Tests ok");
        true
    } else {
        print_fail("Tests failed: ");
        println!(
            "{}",
            str::from_utf8(&out.stderr).expect("cargo test managed to produce invalid utf-8")
        );
        false
    }
}

fn check_untracked_files() -> bool {
    let out = read_cmd_output("git", &["ls-files", "--others", "--exclude-standard"]).unwrap();
    if out.stdout.is_empty() {
        print_ok("No untracked files");
        true
    } else {
        print_fail("Untracked files present:");
        println!("{}", str::from_utf8(&out.stdout).unwrap());
        false
    }
}

fn main() {
    let ok = [format, run_tests, check_untracked_files]
        .into_iter()
        .all(|x| x());
    exit(if ok { 0 } else { 1 });
}
