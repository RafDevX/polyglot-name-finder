use std::{collections::HashMap, error::Error, fs};
use unidecode;

pub fn run(wordlists: Vec<String>) -> Result<(), Box<dyn Error>> {
    let common_words = intersect_wordlists(wordlists)?;

    for word in common_words {
        println!("{word}");
    }

    Ok(())
}

fn intersect_wordlists(wordlists: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut words = HashMap::new();

    for path in wordlists.iter() {
        let contents = fs::read_to_string(path)?;

        for (j, line) in contents.lines().enumerate() {
            handle_line(&mut words, path, j, line)?;
        }

        if words.is_empty() {
            return Ok(Vec::new());
        }
    }

    Ok(words
        .iter()
        .filter(|&(_k, &v)| v == wordlists.len())
        .map(|entry| entry.0.clone())
        .collect())
}

fn handle_line(
    words: &mut HashMap<String, usize>,
    path: &String,
    index: usize,
    line: &str,
) -> Result<(), Box<dyn Error>> {
    let mut line = line.split_whitespace();
    if line.next().is_none() {
        return Err(format!("File {path} has no dice numbers column on line {index}").into());
    }
    if let Some(word) = line.next() {
        let word = normalize_word(word.to_owned());
        if !word.is_empty() {
            let count = words.entry(word).or_insert(0);
            *count += 1;
        }
    } else {
        return Err(format!("File {path} has no words column on line {index}").into());
    };

    Ok(())
}

fn normalize_word(word: String) -> String {
    let mut normalized = String::new();

    for c in word.chars() {
        if c.is_alphabetic() {
            normalized.push(c);
        }
    }

    unidecode::unidecode(&normalized)
}
