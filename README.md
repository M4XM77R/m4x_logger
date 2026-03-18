# Simple Logger

A simple logging utility for Rust applications that supports logging to console, file, or both.

## Features

- Log to console, file, or both
- Automatic timestamp generation
- Thread-safe (Clone implementation)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
simple_logger = { path = "/path/to/simple_logger" }  # For local use
# or
simple_logger = "0.1.0"  # Once published to crates.io
```

## Usage

```rust
use simple_logger::{Logger, LogDestination};

fn main() {
    // Log to console
    let logger = Logger::new(LogDestination::Console);
    logger.log("This is a log message");

    // Log to file
    let logger_file = Logger::new(LogDestination::File("app.log".to_string()));
    logger_file.log("This goes to file");

    // Log to both
    let logger_both = Logger::new(LogDestination::Both("app.log".to_string()));
    logger_both.log("This goes to both console and file");

    // Generate a timestamped filename
    let filename = Logger::generate_timestamp_filename();
    println!("Log file: {}", filename);
}
```

## API

- `Logger::new(destination)`: Create a new logger
- `Logger::log(value)`: Log a debuggable value
- `Logger::generate_timestamp_filename()`: Generate a timestamped filename

## License

MIT