use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Parser)]
#[command(name = "wcr")]
#[command(version = "0.1.0")]
#[command(author = "Wei Shen <shenwei356@gmail.com>")]
#[command(about = "Rust wc")]
struct Cli {
    #[arg(short = 'l', long, help = "Show line count")]
    lines: bool,

    #[arg(short = 'w', long, help = "Show word count")]
    words: bool,

    #[arg(short = 'c', long, help = "Show byte count")]
    bytes: bool,

    #[arg(
        short = 'm',
        long,
        help = "Show character count",
        conflicts_with = "bytes"
    )]
    chars: bool,

    #[arg(default_value = "-", help = "Input file(s)")]
    files: Vec<String>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let mut cli: Cli = Cli::parse();

    if [cli.lines, cli.words, cli.bytes, cli.chars]
        .iter()
        .all(|v| !v)
    {
        cli.lines = true;
        cli.words = true;
        cli.bytes = true;
    }

    Ok(Config {
        files: cli.files,
        lines: cli.lines,
        words: cli.words,
        bytes: cli.bytes,
        chars: cli.chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(&config);
    let mut total_lines: u64 = 0;
    let mut total_words: u64 = 0;
    let mut total_bytes: u64 = 0;
    let mut total_chars: u64 = 0;

    for filename in config.files.iter() {
        // println!("{}", f)
        match open(filename) {
            Err(e) => eprintln!("failed to open {}: {}", filename, e),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    // print!("{:?}", info);
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.lines, config.lines),
                        format_field(info.words, config.words),
                        format_field(info.bytes, config.bytes),
                        format_field(info.chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );

                    total_lines += info.lines;
                    total_words += info.words;
                    total_bytes += info.bytes;
                    total_chars += info.chars;
                }
            }
        }
    }
    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars),
        );
    }
    Ok(())
}

fn format_field(value: u64, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    lines: u64,
    words: u64,
    bytes: u64,
    chars: u64,
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut lines: u64 = 0;
    let mut words: u64 = 0;
    let mut bytes: u64 = 0;
    let mut chars: u64 = 0;

    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)? as u64;
        if line_bytes == 0 {
            break;
        }
        bytes += line_bytes;
        lines += 1;
        words += line.split_whitespace().count() as u64;
        chars += line.chars().count() as u64;
        line.clear()
    }

    Ok(FileInfo {
        lines: lines,
        words: words,
        bytes: bytes,
        chars: chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{FileInfo, count};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        let expected = FileInfo {
            lines: 1,
            words: 10,
            chars: 48,
            bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
