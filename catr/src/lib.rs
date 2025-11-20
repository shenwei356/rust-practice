use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

#[derive(Parser)]
#[command(name = "catr")]
#[command(version = "0.1.0")]
#[command(author = "Wei Shen <shenwei356@gmail.com>")]
#[command(about = "Rust cat")]
struct Cli {
    #[arg(short = 'n', long, help = "Number lines")]
    number: bool,

    #[arg(short = 'b', long, help = "Number nonblank lines")]
    number_nonblank: bool,

    #[arg(default_value = "-", help = "Input file(s)")]
    files: Vec<String>,
}

pub fn get_args() -> MyResult<Config> {
    let cli: Cli = Cli::parse();
    Ok(Config {
        number_lines: cli.number,
        number_nonblank_lines: cli.number_nonblank,
        files: cli.files,
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    for filename in config.files {
        // println!("{}", f)
        match open(&filename) {
            Err(e) => eprintln!("failed to open {}: {}", filename, e),
            Ok(file) => {
                let mut n = 0;
                for line_result in file.lines() {
                    let line = line_result?;

                    if config.number_lines {
                        n += 1;
                        println!("{:>6}\t{}", n, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            n += 1;
                            println!("{:>6}\t{}", n, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
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
