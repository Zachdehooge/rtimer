mod timer;

use clap::Parser;
use crate::timer::run_and_time;

/// Run a shell command and time its execution.
#[derive(Parser)]
struct Args {
    /// Command to run (quoted)
    command: String,
}

fn main() {
    let args = Args::parse();

    match run_and_time(&args.command) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
