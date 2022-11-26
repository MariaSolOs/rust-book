use std::{env, process};

use mongrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Parsing configuration failed: {err}");
        process::exit(1);
    });

    if let Err(err) = mongrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
