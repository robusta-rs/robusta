# Corpus coverage

The parser and formatter are tested against the `misc` fixtures and read-only Java corpora for parse success, formatter idempotency, and significant-token and comment preservation.

## Development

```bash
# full run
cargo coverage

# run a single tool
cargo coverage parser
cargo coverage formatter

# also check one or more corpus roots
cargo coverage /path/to/corpus-a /path/to/corpus-b
cargo coverage formatter /path/to/corpus

# print every failure, which a filter also does
cargo coverage --detail /path/to/corpus

# filter for a file path
cargo coverage parser --filter misc/fail/method-without-body.java

# find crash scene by running on a single worker thread
cargo coverage --debug
```
