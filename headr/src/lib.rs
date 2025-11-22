use assert_cmd::assert;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

#[derive(Parser)]
#[command(name = "headr")]
#[command(version = "0.1.0")]
#[command(author = "Wei Shen <shenwei356@gmail.com>")]
#[command(about = "Rust head")]
struct Cli {
    #[arg(
        short = 'n',
        long,
        default_value = "10",
        help = "Number of lines",
        conflicts_with = "bytes"
    )]
    lines: usize,

    #[arg(short = 'c', long, help = "Number of bytes", conflicts_with = "lines")]
    bytes: Option<usize>,

    #[arg(default_value = "-", help = "Input file(s)")]
    files: Vec<String>,
}

pub fn get_args() -> MyResult<Config> {
    let cli: Cli = Cli::parse();
    Ok(Config {
        lines: cli.lines,
        bytes: cli.bytes,
        files: cli.files,
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    // dbg!(&config);
    for (file_num, filename) in config.files.iter().enumerate() {
        // println!("{}", f)
        match open(&filename) {
            Err(e) => eprintln!("failed to open {}: {}", filename, e),
            Ok(mut file) => {
                // for line in file.lines().take(config.lines) {
                //     println!("{}", line?)
                // }
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear()
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        //  _ => Err(From::from(val)),
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
