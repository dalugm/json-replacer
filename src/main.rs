use std::process;

use clap::Parser;
use json_replacer::{Cli, Config, run};

fn main() {
    let args = Cli::parse();
    let config = Config::new(args);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
