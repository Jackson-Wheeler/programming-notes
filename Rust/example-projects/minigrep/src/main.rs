use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // get configuration - parse command line arguments
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    // run the program
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
