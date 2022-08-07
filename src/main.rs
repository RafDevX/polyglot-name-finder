use std::{env::args, process};

fn main() {
    let wordlists = polyglot_name_finder::Config::new(args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match polyglot_name_finder::run(wordlists) {
        Ok(common_words) => {
            for word in common_words {
                println!("{word}");
            }
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
