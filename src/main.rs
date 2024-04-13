use clap::{App, Arg};
use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;
fn grep<R: BufRead>(read_stream: &mut R, pattern: &str, show_line: bool, case_sensitive: bool) {
    for (line, text) in read_stream.lines().enumerate() {
        let text = match text {
            Ok(text) => text,
            Err(_err) => {
                eprintln!("Error reading line {}", line);
                continue;
            }
        };
        let pattern = if case_sensitive {
            pattern.to_lowercase()
        } else {
            pattern.to_string()
        };
        let text = if case_sensitive {
            text.to_lowercase()
        } else {
            text.to_string()
        };
        if let Some(index) = text.find(&pattern) {
            let before_pattern = &text[..index];
            let after_pattern = &text[index + pattern.len()..];
            let colored_pattern = &text[index..index + pattern.len()];
            let highlit = colored_pattern.bright_red().bold();
            if show_line {
                println!(
                    "{} {}{}{}",
                    line + 1,
                    before_pattern,
                    highlit,
                    after_pattern
                );
            } else {
                println!("{}{}{}", before_pattern, highlit, after_pattern);
            }
        }
    }
}
fn main() {
    let matches = App::new("rgrep")
        .version("0.1")
        .author("Akshay Kumar Ray <akprpa@gmail.com>")
        .about("Searches for pattern in a file or stream")
        .arg(Arg::new("pattern").required(true).index(1))
        .arg(Arg::new("file").required(true).index(2))
        .arg(
            Arg::new("show_line")
                .required(false)
                .short('n')
                .takes_value(false),
        )
        .arg(
            Arg::new("case_insensitive")
                .required(false)
                .short('i')
                .takes_value(false),
        )
        .get_matches();

    let file_path = matches.value_of("file");
    let pattern = matches.value_of("pattern").unwrap();
    let show_line = matches.is_present("show_line");
    let case_insensitive = matches.is_present("case_insensitive");
    let mut reader: Box<dyn BufRead> = if let Some(file_path) = file_path {
        if !Path::new(file_path).exists() {
            eprintln!("Error: File '{}' does not exist.", file_path);
            exit(1);
        }

        match File::open(file_path) {
            Ok(file) => Box::new(BufReader::new(file)),
            Err(err) => {
                eprintln!("Error opening file '{}': {}", file_path, err);
                exit(1);
            }
        }
    } else {
        Box::new(std::io::stdin().lock())
    };
    grep(&mut reader, pattern, show_line, case_insensitive);
}
