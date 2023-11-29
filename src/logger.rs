use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use humantime;

/// A logger instance is represented here
pub struct Logger {
    file: Arc<Mutex<File>>,
}

impl Logger {
    /// Returns a Logger instance
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the absolute or relative
    /// path to the log file
    ///
    /// # Example
    ///
    /// ```ignore
    /// use corrodedweb::logger;
    /// let l = logger::Logger::new("./test.log");
    /// ```
    pub fn new(path: &str) -> Logger {
        let mut log_path = PathBuf::new();
        log_path.push(path);

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)
            .unwrap();

        let file = Arc::new(Mutex::new(file));

        Logger { file }
    }

    pub fn debug(logger: &Option<Logger>, message: &str) {
        if let Some(logger) = logger {
            logger._debug(message);
        }
    }

    pub fn info(logger: &Option<Logger>, message: &str) {
        if let Some(logger) = logger {
            logger._info(message);
        }
    }

    pub fn warning(logger: &Option<Logger>, message: &str) {
        if let Some(logger) = logger {
            logger._warning(message);
        }
    }

    /// Creates a Debug information and passes it to write_to_file
    ///
    /// # Arguments
    ///
    /// * `message` - A reference to a string slice containing the
    /// log message
    ///
    /// # Example
    ///
    /// ```ignore
    /// use corrodedweb::logger;
    /// let l = logger::Logger::new("./test.log");
    /// l.debug("This is the debug message");
    /// ```
    pub fn _debug(&self, message: &str) -> String {
        // Todo pass optional vec with args for debug information
        let mut msg = String::from("DEBUG (");
        let sys_time = self.get_sys_time();
        msg.push_str(&sys_time.as_str());
        msg.push_str("): ");
        msg.push_str(message);
        self.write_to_file(&msg);
        sys_time
    }

    /// Creates a Info information and passes it to write_to_file
    ///
    /// # Arguments
    ///
    /// * `message` - A reference to a string slice containing the
    /// log message
    ///
    /// # Example
    ///
    /// ```ignore
    /// use corrodedweb::logger;
    /// let l = logger::Logger::new("./test.log");
    /// l.info("This is the info message");
    /// ```
    pub fn _info(&self, message: &str) -> String {
        let mut msg = String::from("INFO (");
        let sys_time = self.get_sys_time();
        msg.push_str(&sys_time.as_str());
        msg.push_str("): ");
        msg.push_str(message);
        self.write_to_file(&msg);
        sys_time
    }

    /// Creates a Warning information and passes it to write_to_file
    ///
    /// # Arguments
    ///
    /// * `message` - A reference to a string slice containing the
    /// log message
    ///
    /// # Example
    ///
    /// ```ignore
    /// use corrodedweb::logger;
    /// let l = logger::Logger::new("./test.log");
    /// l.warning("This is the warning message");
    /// ```
    pub fn _warning(&self, message: &str) -> String {
        let mut msg = String::from("WARNING (");
        let sys_time = self.get_sys_time();
        msg.push_str(&sys_time.as_str());
        msg.push_str("): ");
        msg.push_str(message);
        self.write_to_file(&msg);
        sys_time
    }

    fn write_to_file(&self, _message: &str) {
        if let Ok(mut file) = self.file.lock() {
            if let Err(e) = writeln!(file, "{}", _message) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }

    fn get_sys_time(&self) -> String {
        let sys_time = SystemTime::now();
        let timestamp = humantime::format_rfc3339_seconds(sys_time);
        timestamp.to_string()
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Logger {
            file: self.file.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[test]
    fn test_logger() {
        // test creation of logger
        let logger = Logger::new("./test.log");
        let mut path: PathBuf = PathBuf::new();
        path.push("./test.log");
        //assert_eq!(logger.log_path, path);

        let message = "This Test Message";
        let mut sys_time = String::new();

        // Test if file is created when non existant
        if !Path::new("./test.log").exists() {
            assert_eq!(Path::new("./test.log").exists(), false);
            sys_time.push_str(logger._debug(message).as_str());
            assert_eq!(Path::new("./test.log").exists(), true);
        } else {
            sys_time.push_str(logger._debug(message).as_str());
            assert_eq!(Path::new("./test.log").exists(), true);
        }

        // Create compare_msg to compare to line written in test.log
        let mut compare_msg = String::from("DEBUG (");
        compare_msg.push_str(sys_time.as_str());
        compare_msg.push_str("): ");
        compare_msg.push_str(message);

        // Test if last line equals to message written to test.log
        let file = File::open("./test.log").expect("Opening file");
        let content = BufReader::new(&file);
        let lines = content.lines();
        match lines.last() {
            Some(last_line) => {
                match last_line {
                    Ok(extracted_message) => {
                        assert_eq!(compare_msg, extracted_message);
                    }
                    Err(_) => panic!("Something went wrong"),
                };
            }
            None => panic!("Something went wrong"),
        };

        drop(message);
        drop(compare_msg);
        drop(sys_time);

        let message = "This Test Message";
        let mut sys_time = String::new();

        // Test if file is created when non existant
        if !Path::new("./test.log").exists() {
            assert_eq!(Path::new("./test.log").exists(), false);
            sys_time.push_str(logger._info(message).as_str());
            assert_eq!(Path::new("./test.log").exists(), true);
        } else {
            sys_time.push_str(logger._info(message).as_str());
            assert_eq!(Path::new("./test.log").exists(), true);
        }

        // Create compare_msg to compare to line written in test.log
        let mut compare_msg = String::from("INFO (");
        compare_msg.push_str(sys_time.as_str());
        compare_msg.push_str("): ");
        compare_msg.push_str(message);

        // Test if last line equals to message written to test.log
        let file = File::open("./test.log").expect("Opening file");
        let content = BufReader::new(&file);
        let lines = content.lines();
        match lines.last() {
            Some(last_line) => {
                match last_line {
                    Ok(extracted_message) => {
                        assert_eq!(compare_msg, extracted_message);
                    }
                    Err(_) => panic!("Something went wrong"),
                };
            }
            None => panic!("Something went wrong"),
        };

        drop(message);
        drop(compare_msg);
        drop(sys_time);

        let message = "This Test Message";
        let mut sys_time = String::new();

        // Test if file is created when non existant
        if !Path::new("./test.log").exists() {
            assert_eq!(Path::new("./test.log").exists(), false);
            sys_time.push_str(logger._warning(message).as_str());
            assert_eq!(Path::new("./test.log").exists(), true);
        } else {
            sys_time.push_str(logger._warning(message).as_str());
            assert_eq!(Path::new("./test.log").exists(), true);
        }

        // Create compare_msg to compare to line written in test.log
        let mut compare_msg = String::from("WARNING (");
        compare_msg.push_str(sys_time.as_str());
        compare_msg.push_str("): ");
        compare_msg.push_str(message);

        // Test if last line equals to message written to test.log
        let file = File::open("./test.log").expect("Opening file");
        let content = BufReader::new(&file);
        let lines = content.lines();
        match lines.last() {
            Some(last_line) => {
                match last_line {
                    Ok(extracted_message) => {
                        assert_eq!(compare_msg, extracted_message);
                    }
                    Err(_) => panic!("Something went wrong"),
                };
            }
            None => panic!("Something went wrong"),
        };
    }
}
