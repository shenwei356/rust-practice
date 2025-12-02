use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    types: Vec<EntryType>,
}

#[derive(Parser)]
#[command(name = "findr")]
#[command(version = "0.1.0")]
#[command(author = "Wei Shen <shenwei356@gmail.com>")]
#[command(about = "Rust find")]
struct Cli {
    #[arg(num_args = 1.., default_value = ".", help = "Input file")]
    paths: Vec<String>,

    #[arg(short = 'n', long = "name", help = "Name(s)")]
    names: Vec<String>,

    #[arg(short = 't', long = "type", value_parser = ["f", "d", "l"], help = "Entry type(s)")]
    types: Vec<String>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let cli: Cli = Cli::parse();

    let names = cli
        .names
        .iter()
        .map(|name| Regex::new(&name).map_err(|_| format!("error: invalid value '{}'", name)))
        .collect::<Result<Vec<_>, _>>()?;

    let types = cli
        .types
        .iter()
        .map(|val| match val.as_str() {
            "d" => EntryType::Dir,
            "f" => EntryType::File,
            "l" => EntryType::Link,
            _ => unreachable!("Invalid type"),
        })
        .collect();

    Ok(Config {
        paths: cli.paths,
        names: names,
        types: types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(&config);
    let fype_filter = |entry: &DirEntry| {
        config.types.is_empty()
            || config.types.iter().any(|entry_type| match entry_type {
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
                EntryType::Link => entry.file_type().is_symlink(),
            })
    };

    let name_filter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(fype_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
