//! This module contains the ANSI escape codes for text formatting.
//! The ANSI escape codes are used to format the output of the program.

/// AnsiCode is a trait that requires the implementation of the `ansi` method.
/// # Example
/// ```
/// use resplit::ansi::*;
/// println!("{}This is red text!{}", ForegroundColor::Red.ansi(), Commands::Reset.ansi());
/// ```
/// ```
/// use resplit::ansi::*;
///     let line_format_a =
///         ForegroundColor::Red.ansi().clone() +
///         &BackgroundColor::BrightYellow.ansi() +
///         &Decoration::Bold.ansi();
///
///     let reset_colors = Commands::Reset.ansi();
///
///     println!("{}This is red text on a bright yellow background!{}", line_format_a, reset_colors);
/// ```
/// ForegroundColor represents the ANSI color codes for foreground colors.

#[derive(Copy, Clone)]
pub enum ForegroundColor {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
}

/// BackgroundColor represents the ANSI color codes for background colors.

#[derive(Copy, Clone)]
pub enum BackgroundColor {
    Black = 40,
    Red = 41,
    Green = 42,
    Yellow = 43,
    Blue = 44,
    Magenta = 45,
    Cyan = 46,
    White = 47,
    BrightBlack = 100,
    BrightRed = 101,
    BrightGreen = 102,
    BrightYellow = 103,
    BrightBlue = 104,
    BrightMagenta = 105,
    BrightCyan = 106,
    BrightWhite = 107,
}

/// Decoration represents the ANSI color codes for text decorations.

#[derive(Copy, Clone)]
pub enum Decoration {
    Bold = 1,
    Dim = 2,
    Italic = 3,
    Underline = 4,
    Blink = 5,
    Reverse = 7,
    Hidden = 8,
    Strikethrough = 9,
}

/// Commands represents the ANSI codes for command operations.

#[derive(Copy, Clone)]
pub enum Commands {
    Reset = 0,
    Clear = 2,
}

/// AnsiCode is a trait that requires the implementation of the `ansi` method.
/// The `ansi` method should return a String that represents the ANSI escape code.

pub trait AnsiCode {
    fn ansi(&self) -> String;
}

/// Implementation of AnsiCode for ForegroundColor.
/// The `ansi` method returns the ANSI escape code for the foreground color.

impl AnsiCode for ForegroundColor {
    fn ansi(&self) -> String {
        format!("\x1b[{}m", *self as u32)
    }
}

/// Implementation of AnsiCode for BackgroundColor.
/// The `ansi` method returns the ANSI escape code for the background color.

impl AnsiCode for BackgroundColor {
    fn ansi(&self) -> String {
        format!("\x1b[{}m", *self as u32)
    }
}

/// Implementation of AnsiCode for Decoration.
/// The `ansi` method returns the ANSI escape code for the text decoration.

impl AnsiCode for Decoration {
    fn ansi(&self) -> String {
        format!("\x1b[{}m", *self as u32)
    }
}

/// Implementation of AnsiCode for Commands.
/// The `ansi` method returns the ANSI escape code for the command operation.

impl AnsiCode for Commands {
    fn ansi(&self) -> String {
        match self {
            Commands::Reset => format!("\x1b[{}m", *self as u32),
            Commands::Clear => format!("\x1b[{}J", *self as u32),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi_reset() {
        let command = Commands::Reset;
        assert_eq!(command.ansi(), format!("\x1b[{}m", command as u32));
    }

    #[test]
    fn test_ansi_clear() {
        let command = Commands::Clear;
        assert_eq!(command.ansi(), format!("\x1b[{}J", command as u32));
    }
}
