use std::{collections::HashMap, env, error::Error, fs};

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let mut common_words = intersect_wordlists(
        config.wordlist_file_paths,
        config.min_word_length,
        config.require_diff_letters,
    )?;

    if config.sort {
        common_words.sort_unstable();
    }

    Ok(common_words)
}

pub struct Config {
    wordlist_file_paths: Vec<String>,
    min_word_length: usize,
    require_diff_letters: bool,
    sort: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let wordlist_file_paths: Vec<String> = args.collect();

        let min_word_length = env::var("MIN_WORD_LENGTH")
            .map_or(Ok(1), |x| x.parse())
            .map_err(|_| "Failed to parse MIN_WORD_LENGTH")?;

        let require_diff_letters = env::var("REQUIRE_DIFF_LETTERS").is_ok();
        let sort = !env::var("NO_SORT").is_ok();

        if wordlist_file_paths.is_empty() {
            Err("No wordlist files provided")
        } else {
            Ok(Config {
                wordlist_file_paths,
                min_word_length,
                require_diff_letters,
                sort,
            })
        }
    }
}

fn intersect_wordlists(
    wordlists: Vec<String>,
    min_word_length: usize,
    require_diff_letters: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut words = HashMap::new();

    for path in wordlists.iter() {
        let contents = fs::read_to_string(path)?;

        for (j, line) in contents.lines().enumerate() {
            handle_line(
                &mut words,
                min_word_length,
                require_diff_letters,
                path,
                j,
                line,
            )?;
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
    min_word_length: usize,
    require_diff_letters: bool,
    path: &String,
    index: usize,
    line: &str,
) -> Result<(), Box<dyn Error>> {
    let word = line.split_whitespace().last();

    if let Some(word) = word {
        let word = normalize_word(word.to_owned());
        if word.len() >= min_word_length
            && !(require_diff_letters && word.chars().min() == word.chars().max())
        {
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

    let normalized = normalized.to_lowercase();

    unidecode::unidecode(&normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_words() {
        let words = [
            ["", ""],
            ["&&", ""],
            ["^", ""],
            ["normal", "normal"],
            ["æ52m⁴ô", "aemo"],
            ["Ÿ£€$y ã", "yya"],
        ];

        for [word, normalized] in words {
            assert_eq!(normalized, normalize_word(word.to_owned()));
        }
    }
}
