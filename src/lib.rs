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
//! - Thread-safe (Clone implementation)
//!
//! ## Example
//!
//! ```rust
//! use m4x_logger::{Logger, LogDestination};
//!
//! let logger = Logger::new(LogDestination::Console);
//! logger.log("This is a log message");
//!
//! let logger_file = Logger::new(LogDestination::File("app.log".to_string()));
//! logger_file.log("This goes to file");
//!
//! let logger_both = Logger::new(LogDestination::Both("app.log".to_string()));
//! logger_both.log("This goes to both console and file");
//! ```

use chrono::Local;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;

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

/// A simple logger that can write messages to various destinations with timestamps.
#[derive(Clone)]
pub struct Logger {
    destination: LogDestination,
}

impl Logger {
    /// Creates a new Logger with the specified destination.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_logger::{Logger, LogDestination};
    ///
    /// let logger = Logger::new(LogDestination::Console);
    /// ```
    pub fn new(destination: LogDestination) -> Self {
        Self { destination }
    }

    /// Generates a timestamped filename for log files.
    ///
    /// The format is `DD:MM:YYYY:HH:MM:SS.log`.
    ///
    /// # Examples
    ///
    /// ```
    /// let filename = simple_logger::Logger::generate_timestamp_filename();
    /// println!("Log file: {}", filename);
    /// ```
    pub fn generate_timestamp_filename() -> String {
        let now = Local::now();
        let timestamp = now.format("%d:%m:%Y:%H:%M:%S");
        format!("{}.log", timestamp)
    }

    /// Logs a debuggable value with a timestamp.
    ///
    /// The message format is: `[DD:MM:YYYY HH:MM:SS] [LOG] {value:?}\n`
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_logger::{Logger, LogDestination};
    ///
    /// let logger = Logger::new(LogDestination::Console);
    /// logger.log("Hello, world!");
    /// logger.log(42);
    /// ```
    pub fn log<T: Debug>(&self, var: T) {
        let now = Local::now();
        let timestamp = now.format("%d:%m:%Y %H:%M:%S").to_string();
        let message = format!("[{timestamp}] [LOG] {:?}\n", var);

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
}