use std::env;
use std::process;

use ioProject::Config;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = ioProject::run(config) {
        // --snip--
        println!("Application error: {}", e);

        process::exit(1);
    }
}
