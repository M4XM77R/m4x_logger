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
use m4x_logger::{Logger, LogDestination, LogLevel};

fn main() {
    // Test explicit log level
    let logger_explicit = Logger::new(LogDestination::Console, LogLevel::Info);
    logger_explicit.log(LogLevel::Info, "This is a test log message with explicit Info level.");
    logger_explicit.error("This is an error message.");
    logger_explicit.warn("This is a warning message.");
    logger_explicit.info("This is an info message.");
    logger_explicit.debug("This debug message won't show (level is Info).");
    logger_explicit.trace("This trace message won't show (level is Info).");

    println!("\n--- Testing automatic level selection ---");

    // Test automatic level selection based on build mode
    let logger_auto = Logger::with_destination(LogDestination::Console);
    println!("Default level: {:?}", Logger::default_level());
    logger_auto.error("This is an error message (should always show).");
    logger_auto.warn("This is a warning message.");
    logger_auto.info("This is an info message.");
    logger_auto.debug("This is a debug message.");
    logger_auto.trace("This is a trace message.");

    println!("\n--- Testing file logging ---");

    // Test file logging that should NOT create files (filtered out)
    let logger_file_filtered = Logger::new(LogDestination::File("test_filtered.log".to_string()), LogLevel::Error);
    logger_file_filtered.info("This info message should not create a file.");
    logger_file_filtered.debug("This debug message should not create a file.");

    // Test file logging that SHOULD create files
    let logger_file_created = Logger::new(LogDestination::File("test_created.log".to_string()), LogLevel::Info);
    logger_file_created.error("This error message should create a file.");
    logger_file_created.info("This info message should also be in the file.");
}