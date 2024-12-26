use clap::Parser;
use std::error::Error;

mod input;
mod problems;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Config {
    #[arg(short, long)]
    day: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::parse();
    problems::solve(cfg.day)
}
