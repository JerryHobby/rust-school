use log::{Logging, LogLevel, LogOutput};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_debug(){
        let mut log = Logging::new();
        log.enable();
        log.set_level(LogLevel::Debug);
        log.set_output(LogOutput::Stdout);
        log.log(LogLevel::Debug, "This is a debug message");
    }
    #[test]
    fn test_logging_info(){
        let mut log = Logging::new();
        log.enable();
        log.set_level(LogLevel::Info);
        log.set_output(LogOutput::Stdout);
        log.log(LogLevel::Debug, "This is an Info message");
    }

    #[test]
    fn test_logging_disabled(){
        let mut log = Logging::new();
        //log.enable();
        log.set_level(LogLevel::Info);
        log.set_output(LogOutput::Stdout);
        log.log(LogLevel::Debug, "This is disabled message");
    }
}