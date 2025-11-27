use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

#[derive(Parser)]
#[command(name = "uniqr")]
#[command(version = "0.1.0")]
#[command(author = "Wei Shen <shenwei356@gmail.com>")]
#[command(about = "Rust wc")]
struct Cli {
    #[arg(short = 'c', long, help = "Show counts")]
    count: bool,

    #[arg(default_value = "-", help = "Input file")]
    in_file: String,

    #[arg(help = "Output file")]
    out_file: Option<String>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let cli: Cli = Cli::parse();

    Ok(Config {
        in_file: cli.in_file,
        out_file: cli.out_file,
        count: cli.count,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(&config);
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    let mut print = |count: u64, txt: &String| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, txt)?;
            } else {
                write!(out_file, "{}", txt)?;
            }
        }
        Ok(())
    };

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    print(count, &previous)?;

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
