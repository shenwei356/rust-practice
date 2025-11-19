use clap::Parser;

#[derive(Parser)]
#[command(name = "echor")]
#[command(version = "0.1.0")]
#[command(author = "Wei Shen <shenwei356@gmail.com>")]
#[command(about = "Rust echo")]
struct Cli {
    #[arg(short, long, help = "Do not print newline in the end")]
    no_new_line: bool,

    #[arg(
        short,
        long,
        default_value = " ",
        help = "Separator between input texts"
    )]
    separator: String,

    #[arg(short, long, action = clap::ArgAction::Count, help = "Verbose level")]
    verbose: u8,

    #[arg(num_args = 1.., help = "input texts")]
    texts: Vec<String>,
}

fn main() {
    let cli: Cli = Cli::parse();

    // let mut ending = "\n";
    // if cli.no_new_line {
    //     ending = "";
    // }
    let ending: &str = if cli.no_new_line { "" } else { "\n" };

    print!("{}{}", cli.texts.join(&cli.separator), ending)
}
