use std::process;

use clap::Parser;
use json_replacer::{Cli, run};

fn main() {
    let args = Cli::parse();

    if let Err(e) = run(args) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
