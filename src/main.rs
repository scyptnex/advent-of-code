use clap::Parser;
use std::error::Error;

mod problems;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Config {
    #[arg(short, long)]
    day: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::parse();
    println!("Advent day={}", cfg.day);
    let p = problems::get(cfg.day)?;
    p.solve();
    Ok(())
}
