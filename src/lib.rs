// Copyright 2026 Max
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! # Simple Logger Crate
//!
//! A simple logging utility for Rust applications that supports logging to console, file, or both.
//!
//! ## Features
//!
//! - Log to console, file, or both
//! - Automatic timestamp generation
//! - Configurable log levels (Error, Warn, Info, Debug, Trace)
//! - Automatic log level based on build mode (Trace in debug, Error in release)
//! - Thread-safe (Clone implementation)
//!
//! ## Example
//!
//! ```rust
//! use m4x_logger::{Logger, LogDestination, LogLevel};
//!
//! let logger = Logger::new(LogDestination::Console, LogLevel::Info);
//! logger.info("This is an info message");
//! logger.error("This is an error message");
//! logger.debug("This debug message won't be shown");
//!
//! let logger_file = Logger::new(LogDestination::File("app.log".to_string()), LogLevel::Debug);
//! logger_file.debug("This goes to file");
//!
//! let logger_both = Logger::new(LogDestination::Both("app.log".to_string()), LogLevel::Warn);
//! logger_both.warn("This goes to both console and file");
//!
//! // Using default level based on build mode
//! let logger_default = Logger::with_destination(LogDestination::Console);
//! logger_default.info("This uses Trace level in debug builds, Error level in release");
//! ```

use chrono::Local;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;

/// Represents the log level for filtering messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Only error messages
    Error = 0,
    /// Warning and error messages
    Warn = 1,
    /// Info, warning, and error messages
    Info = 2,
    /// Debug, info, warning, and error messages
    Debug = 3,
    /// All messages including trace
    Trace = 4,
}

/// Represents the destination for log messages.
#[derive(Clone)]
pub enum LogDestination {
    /// No logging
    None,
    /// Log to console only
    Console,
    /// Log to file only (provide filename)
    File(String),
    /// Log to both console and file (provide filename)
    Both(String),
}

/// A simple logger that can write messages to various destinations with timestamps and log levels.
#[derive(Clone)]
pub struct Logger {
    destination: LogDestination,
    level: LogLevel,
}

impl Logger {
    /// Creates a new Logger with the specified destination and minimum log level.
    ///
    /// # Examples
    ///
    /// ```
    /// use m4x_logger::{Logger, LogDestination, LogLevel};
    ///
    /// let logger = Logger::new(LogDestination::Console, LogLevel::Info);
    /// ```
    pub fn new(destination: LogDestination, level: LogLevel) -> Self {
        Self { destination, level }
    }

    /// Creates a new Logger with the specified destination and default log level.
    /// In debug builds (dev mode), the default level is Trace. In release builds, it's Error.
    ///
    /// # Examples
    ///
    /// ```
    /// use m4x_logger::{Logger, LogDestination};
    ///
    /// let logger = Logger::with_destination(LogDestination::Console);
    /// ```
    pub fn with_destination(destination: LogDestination) -> Self {
        Self::new(destination, Self::default_level())
    }

    /// Returns the default log level based on build mode.
    /// - Debug builds: Trace (all messages)
    /// - Release builds: Error (only errors)
    ///
    /// # Examples
    ///
    /// ```
    /// use m4x_logger::Logger;
    ///
    /// let level = Logger::default_level();
    /// ```
    pub fn default_level() -> LogLevel {
        #[cfg(debug_assertions)]
        return LogLevel::Trace;
        
        #[cfg(not(debug_assertions))]
        return LogLevel::Info;
    }

    /// Generates a timestamped filename for log files.
    ///
    /// The format is `DD:MM:YYYY:HH:MM:SS.log`.
    ///
    /// # Examples
    ///
    /// ```
    /// let filename = m4x_logger::Logger::generate_timestamp_filename();
    /// println!("Log file: {}", filename);
    /// ```
    pub fn generate_timestamp_filename() -> String {
        let now = Local::now();
        let timestamp = now.format("%d.%m.%Y.%H.%M.%S");
        format!("{}.log", timestamp)
    }

    /// Logs a debuggable value with a timestamp and specified log level.
    ///
    /// The message format is: `[DD:MM:YYYY HH:MM:SS] [LEVEL] {value:?}\n`
    ///
    /// # Examples
    ///
    /// ```
    /// use m4x_logger::{Logger, LogDestination, LogLevel};
    ///
    /// let logger = Logger::new(LogDestination::Console, LogLevel::Info);
    /// logger.log(LogLevel::Info, "Hello, world!");
    /// logger.log(LogLevel::Error, 42);
    /// ```
    pub fn log<T: Debug>(&self, level: LogLevel, var: T) {
        if level > self.level {
            return;
        }

        let now = Local::now();
        let timestamp = now.format("%d:%m:%Y %H:%M:%S").to_string();
        let level_str = match level {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        };
        let message = format!("[{timestamp}] [{level_str}] {:?}\n", var);

        match &self.destination {
            LogDestination::None => {
                // Do nothing
            }
            LogDestination::Console => {
                print!("{}", message);
            }
            LogDestination::File(filename) => {
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(filename)
                {
                    let _ = file.write_all(message.as_bytes());
                }
            }
            LogDestination::Both(filename) => {
                print!("{}", message);
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(filename)
                {
                    let _ = file.write_all(message.as_bytes());
                }
            }
        }
    }

    /// Logs an error message.
    ///
    /// # Examples
    ///
    /// ```
    /// use m4x_logger::{Logger, LogDestination, LogLevel};
    ///
    /// let logger = Logger::new(LogDestination::Console, LogLevel::Info);
    /// logger.error("This is an error");
    /// ```
    pub fn error<T: Debug>(&self, var: T) {
        self.log(LogLevel::Error, var);
    }

    /// Logs a warning message.
    ///
    /// # Examples
    ///
    /// ```
    /// use m4x_logger::{Logger, LogDestination, LogLevel};
    ///
    /// let logger = Logger::new(LogDestination::Console, LogLevel::Info);
    /// logger.warn("This is a warning");
    /// ```
    pub fn warn<T: Debug>(&self, var: T) {
        self.log(LogLevel::Warn, var);
    }

    /// Logs an info message.
    ///
    /// # Examples
    ///
    /// ```
    /// use m4x_logger::{Logger, LogDestination, LogLevel};
    ///
    /// let logger = Logger::new(LogDestination::Console, LogLevel::Info);
    /// logger.info("This is an info message");
    /// ```
    pub fn info<T: Debug>(&self, var: T) {
        self.log(LogLevel::Info, var);
    }

    /// Logs a debug message.
    ///
    /// # Examples
    ///
    /// ```
    /// use m4x_logger::{Logger, LogDestination, LogLevel};
    ///
    /// let logger = Logger::new(LogDestination::Console, LogLevel::Debug);
    /// logger.debug("This is a debug message");
    /// ```
    pub fn debug<T: Debug>(&self, var: T) {
        self.log(LogLevel::Debug, var);
    }

    /// Logs a trace message.
    ///
    /// # Examples
    ///
    /// ```
    /// use m4x_logger::{Logger, LogDestination, LogLevel};
    ///
    /// let logger = Logger::new(LogDestination::Console, LogLevel::Trace);
    /// logger.trace("This is a trace message");
    /// ```
    pub fn trace<T: Debug>(&self, var: T) {
        self.log(LogLevel::Trace, var);
    }
}