use clap::Parser;
use repository::*;

fn main() {
    let cli = cli::Cli::parse();

    println!("{:?}", cli);
}
