//! Simple file logger utility for appending timestamped logs

use std::fs::{OpenOptions};
use std::io::{Write, Result};
use std::sync::{Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Thread-safe file logger
pub struct FileLogger {
    file_path: String,
    lock: Mutex<()>, // Ensures multi-thread write safety
}

impl FileLogger {
    /// Create logger with file path
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            file_path: path.into(),
            lock: Mutex::new(()),
        }
    }

    /// Log a raw message
    pub fn log(&self, message: &str) -> Result<()> {
        let _guard = self.lock.lock().unwrap();

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        writeln!(file, "{}", message)?;
        Ok(())
    }

    /// Log a timestamped message
    pub fn log_with_timestamp(&self, message: &str) -> Result<()> {
        let ts = Self::timestamp();
        self.log(&format!("[{}] {}", ts, message))
    }

    /// Unix timestamp seconds
    fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::FileLogger;
    use std::fs;

    #[test]
    fn writes_log_message() {
        let file = "test_log.txt";
        let logger = FileLogger::new(file);

        logger.log("Hello Logger").unwrap();

        let content = fs::read_to_string(file).unwrap();
        assert!(content.contains("Hello Logger"));

        let _ = fs::remove_file(file);
    }

    #[test]
    fn writes_timestamped_log() {
        let file = "test_log_ts.txt";
        let logger = FileLogger::new(file);

        logger.log_with_timestamp("Event happened").unwrap();

        let content = fs::read_to_string(file).unwrap();
        assert!(content.contains("Event happened"));
        assert!(content.contains("[")); // timestamp format

        let _ = fs::remove_file(file);
    }
}
