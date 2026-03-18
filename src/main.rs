use simple_logger::{Logger, LogDestination};

fn main() {
    let logger = Logger::new(LogDestination::Console);
    logger.log("This is a test log message.");
}