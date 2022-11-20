use std::{env, process};

use mongrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Parsing configuration failed: {err}");
        process::exit(1);
    });

    if let Err(err) = mongrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
