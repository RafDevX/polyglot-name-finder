# Polyglot Name Finder

Helps find a name that is a word in multiple languages.

## Building

```sh
cargo build --release
```

## Usage

Assuming your wordlists are wordlists at `wordlists/`:

```sh
MIN_WORD_LENGTH=3 target/release/polyglot_name_finder wordlists/*.txt
```

Use the `MIN_WORD_LENGTH` environment variable to filter out words you consider too short.

In order to, for example, support diceware wordlists, only the last whitespace-separated column is considered.

Words are normalized to increase the chance of finding matches.
