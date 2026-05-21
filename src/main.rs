// use clap::{ArgGroup, Parser};
use noctavox::addons::parse_args;

fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1") };

    if std::env::args().len() == 1 {
        match noctavox::app_core::NoctaVox::new() {
            Ok(mut app) => app.run(),
            Err(e) => eprintln!("{e}"),
        }
        return;
    }

    parse_args();
}
