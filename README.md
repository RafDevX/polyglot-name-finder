# Polyglot Name Finder

Helps find a name that is a word in multiple languages.

## Building

```sh
cargo build --release
```

## Usage

Assuming your wordlists are located in `wordlists/`:

```sh
MIN_WORD_LENGTH=3 target/release/polyglot-name-finder wordlists/*.txt
```

Use the `MIN_WORD_LENGTH` environment variable to filter out words you consider too short. Defaults to `1`.

Set the `REQUIRE_DIFF_LETTERS` if you only want to consider words where not all the characters are equal.

Set the `NO_SORT` environment variable to disable sorting the results (might be faster).

In order to, for example, support diceware wordlists, only the last whitespace-separated column is considered.

Words are normalized to increase the chance of finding matches.

_**NOTE:** Besides building and running in separate steps, you can also use `ENV_VARS=123 cargo run wordlists/*.txt`._
