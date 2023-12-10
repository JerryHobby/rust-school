//! # Log
//! Create a simple logging system
//! ## Example
//! ```
//! use log::{Logging, LogLevel, LogOutput};
//! let mut log = Logging::new();
//! log.enable();
//! log.set_level(LogLevel::Debug);
//! log.set_output(LogOutput::Stdout);
//! log.log(LogLevel::Debug, "This is a debug message");
//! ```

/// # LogLevel
/// The level of the log message

#[allow(dead_code)]
pub enum LogLevel{
    Debug,
    Info,
    Warn,
    Error,
}

/// # LogOutput
/// Where the log message is sent

#[allow(dead_code)]
pub enum LogOutput{
    Stdout,
    Stderr,
    File,
}

/// # Logging
/// The logging system
/// ## Example
/// ```
/// use log::{Logging, LogLevel, LogOutput};
/// let mut log = Logging::new();
/// log.enable();
/// log.set_level(LogLevel::Debug);
/// log.set_output(LogOutput::Stdout);
/// log.log(LogLevel::Debug, "This is a debug message");
/// ```

pub struct Logging{
    enabled: bool,
    level: LogLevel,
    output: LogOutput,
}


#[allow(dead_code)]
impl Logging {
    pub fn new() -> Logging{
        Logging{
            enabled: false,
            level: LogLevel::Info,
            output: LogOutput::Stdout,
        }
    }
    pub fn enable(&mut self){
        self.enabled = true;
    }
    pub fn disable(&mut self){
        self.enabled = false;
    }
    pub fn set_level(&mut self, level: LogLevel){
        self.level = level;
    }
    pub fn set_output(&mut self, output: LogOutput){
        self.output = output;
    }
    pub fn log(&self, level: LogLevel, message: &str){
        if self.enabled{
            let message = match level{
                LogLevel::Debug => format!("[DEBUG] {}", message),
                LogLevel::Info => format!("[INFO] {}", message),
                LogLevel::Warn => format!("[WARN] {}", message),
                LogLevel::Error => format!("[ERROR] {}", message),
            };
            match self.output{
                LogOutput::Stdout => println!("{}", message),
                LogOutput::Stderr => eprintln!("{}", message),
                LogOutput::File => println!("{}", message),
            }
        }
    }
}

