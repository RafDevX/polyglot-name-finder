use std::{env::args, process};

fn main() {
    let wordlists = parse_args(args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = polyglot_name_finder::run(wordlists) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Vec<String>, &'static str> {
    args.next();

    let wordlist_file_paths: Vec<String> = args.collect();

    if wordlist_file_paths.is_empty() {
        Err("No wordlist files provided")
    } else {
        Ok(wordlist_file_paths)
    }
}
