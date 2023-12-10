#[cfg(test)]
mod tests {
    use resplit::ansi::*;
    use resplit::split_line;
    use resplit::Cli;
    use std::io::BufRead;
    use std::io::Cursor;

    #[test]

    pub fn test_samples_resplit() {
        let cli: Cli = Cli::new_test(1, ":".to_string(), "sample_resplit.txt".to_string(), false);

        let clear_screen = Commands::Clear.ansi();
        let reset_colors = Commands::Reset.ansi();

        let line_format_a = ForegroundColor::Red.ansi().clone()
            + &BackgroundColor::BrightYellow.ansi()
            + &Decoration::Bold.ansi();

        let line_format_b = ForegroundColor::BrightCyan.ansi().clone()
            + &BackgroundColor::Black.ansi()
            + &Decoration::Underline.ansi()
            + &Decoration::Bold.ansi();

        let lines = fake_read_stdin();

        for line in lines {
            if line.is_empty() {
                continue;
            }

            print!("{}", clear_screen);
            println!("{}line: {}{}", line_format_a, line, reset_colors);

            let result = split_line(line, &cli);

            println!("{}{}{}", line_format_b, result, reset_colors);
        }
    }

    pub fn fake_read_stdin() -> Vec<String> {
        let mut reader = Cursor::new(b"1:one\n2:two\n3:three\n4:four\n5:five\n");
        // let stdin = std::io::stdin();

        let mut lines = vec![String::new()];

        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).unwrap();
            if bytes_read == 0 {
                break;
            }
            lines.push(line.trim().to_string());
        }
        lines
    }
}
