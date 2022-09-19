use std::env;
use std::process;

use minigrep::Config;
fn main() {
    // the args() function will give us an iterator over the arguments
    // passed to our program
    // The collect function turns that iterator into a collection
    // So we give args the type Vec<String> to tell collect what
    // type of collection we want
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        // print to the standard error stream
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
