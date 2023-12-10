//! Resplit is a demonstration of a command line utility that can be used as a library.
//! It allows you to pass a string and a delimiter and return the nth field.
//! You can also pass a debug flag to see the parts of the string and the length of the parts.
//! It accepts input via stdin.

pub mod ansi;
use clap::Parser;
use std::io::{BufRead, BufReader};
// cargo --lib .

#[derive(Parser)]
#[command(name = "resplit")]
#[command(version = "0.1.0")]
#[command(about = "Split a string on a delimiter and return the nth field")]
#[command(author = "David M. Stauffer")]
//#[command(usage = "resplit -f 2 -d ,")]

/// This is the Cli struct. It is used to parse command line arguments.
/// It is private to the library.

pub struct Cli {
    #[arg(short('c'))]
    pub column: usize,
    #[arg(short('d'))]
    pub delimiter: String,
    #[arg(short('f'))]
    pub filename: String,
    #[arg(long)]
    debug: bool,
}

impl Cli {
    /// This is the new function. It is used to create a new Cli struct.
    /// It is private to the library.

    pub fn new() -> Self {
        Self::parse()
    }

    pub fn new_test(column: usize, delimiter: String, filename: String, debug: bool) -> Self {
        Cli {
            column,
            delimiter,
            filename,
            debug,
        }
    }

    /// This is the parse function. It is used to parse command line arguments.
    /// It is private to the library.

    pub fn parse() -> Self {
        Cli::parse_from(std::env::args())
    }
}

/// This function reads stdin and returns a String.
/// This is used to get input from the user.

pub fn read_stdin(cli: &Cli) -> Vec<String> {
    read_file(cli)
    // let stdin = std::io::stdin();
    // let mut reader = BufReader::new(stdin.lock());
    // let mut lines = vec![String::new()];

    // loop {
    //     let mut line = String::new();
    //     let bytes_read = reader.read_line(&mut line).unwrap();
    //     if bytes_read == 0 {
    //         break;
    //     }
    //     lines.push(line.trim().to_string());
    // }
    // lines
}

use std::fs::File;
use std::path::Path;

fn read_file(cli: &Cli) -> Vec<String> {
    let path = Path::new(cli.filename.as_str());
    let file = File::open(&path);

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut reader = BufReader::new(file);

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

/// This function splits a string on a delimiter and returns the nth field.

pub fn split_line(s: String, cli: &Cli) -> String {
    let parts: Vec<&str> = s.split(&cli.delimiter).collect();

    if cli.debug {
        println!("parts: {:?}", parts);
        println!("parts.len(): {:?}", parts.len());
    }
    parts.get(cli.column).unwrap_or(&"").to_string()
}

// fn main() {
//     let args = Cli::new();
//     println!("Hello, world!");
// }
