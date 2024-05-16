use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // get configuration - parse command line arguments
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Command line parsing error: {}", err);
        process::exit(1);
    });

    // run the program
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
