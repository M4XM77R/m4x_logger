# Simple Logger

A simple vibe coded logging utility for Rust applications that supports logging to console, file, or both.

## Features

- Log to console, file, or both
- Automatic timestamp generation
- Configurable log levels (Error, Warn, Info, Debug, Trace)
- Thread-safe (Clone implementation)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
m4x_logger = { path = "/path/to/m4x_logger" }  # For local use
# or
m4x_logger = "0.1.0"  # Once published to crates.io
```

## Usage

```rust
use m4x_logger::{Logger, LogDestination, LogLevel};

fn main() {
    // Create logger with Info level (shows Info, Warn, Error)
    let logger = Logger::new(LogDestination::Console, LogLevel::Info);

    // Convenience methods
    logger.error("This is an error message");
    logger.warn("This is a warning message");
    logger.info("This is an info message");
    logger.debug("This debug message won't show");
    logger.trace("This trace message won't show");

    // Or use the generic log method
    logger.log(LogLevel::Debug, "This is a debug message");

    // Log to file with Debug level (shows all messages)
    let logger_file = Logger::new(LogDestination::File("app.log".to_string()), LogLevel::Debug);
    logger_file.debug("This goes to file");

    // Log to both with Warn level (shows Warn, Error)
    let logger_both = Logger::new(LogDestination::Both("app.log".to_string()), LogLevel::Warn);
    logger_both.warn("This goes to both console and file");

    // Generate a timestamped filename
    let filename = Logger::generate_timestamp_filename();
    println!("Log file: {}", filename);
}
```

## Log Levels

- `Error`: Only error messages
- `Warn`: Warning and error messages
- `Info`: Info, warning, and error messages (default)
- `Debug`: Debug, info, warning, and error messages
- `Trace`: All messages including trace

## API

- `Logger::new(destination, level)`: Create a new logger with specified destination and log level
- `Logger::with_destination(destination)`: Create a new logger with default Info level
- `Logger::log(level, value)`: Log a debuggable value with specified level
- `Logger::error(value)`, `Logger::warn(value)`, `Logger::info(value)`, `Logger::debug(value)`, `Logger::trace(value)`: Convenience methods for each log level
- `Logger::generate_timestamp_filename()`: Generate a timestamped filename

## License

MIT
