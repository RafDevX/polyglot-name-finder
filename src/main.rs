use std::{env::args, process};

fn main() {
    let wordlists = polyglot_name_finder::Config::new(args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = polyglot_name_finder::run(wordlists) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
