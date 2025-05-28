# minigrep

A simple Rust library for searching text within files, supporting case-sensitive and case-insensitive queries.

* Search for a query string in a text file.

* Case-insensitive search enabled via environment variable IGNORE_CASE.

* Easy configuration using the Config struct.

* Clean, minimal API for embedding or CLI use.

## Usage

```rust
use minigrep::{Config, run};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::build(&args)?;
    run(config)?;
    Ok(())
}
```

Environment Variable

* IGNORE_CASE: Set to any value to make searches case-insensitive.