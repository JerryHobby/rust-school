use log::*;
use resplit::ansi::*;
use resplit::*;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn resplit_main() {
    let clear_screen = Commands::Clear.ansi();
    let reset_colors = Commands::Reset.ansi();

    let line_format_a = ForegroundColor::Red.ansi().clone()
        + &BackgroundColor::BrightYellow.ansi()
        + &Decoration::Bold.ansi();

    let line_format_b = ForegroundColor::BrightCyan.ansi().clone()
        + &BackgroundColor::Black.ansi()
        + &Decoration::Underline.ansi()
        + &Decoration::Bold.ansi();

    //print!("{}", clear_screen);

    let cli = Cli::parse();

    let mut log = Logging::new();
    log.enable();
    log.set_level(LogLevel::Debug);
    log.set_output(LogOutput::Stdout);

    log.log(LogLevel::Debug, "This is a debug message");
    log.log(LogLevel::Info, "This is an info message");
    log.log(LogLevel::Warn, "This is a warning message");
    log.log(LogLevel::Error, "This is an error message");

    let lines = read_stdin(&cli);

    for line in lines {
        if line.is_empty() {
            continue;
        }

        println!("{}line: {}{}", line_format_a, line, reset_colors);
        let result = split_line(line, &cli);
        println!("{}{}{}", line_format_b, result, reset_colors);
    }
}
