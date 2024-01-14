mod elf;

use crate::elf::load;
use clap::Parser;
use std::error;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value_t = String::from("a.out"))]
    file: String,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    let elf = load(args.file.into())?;

    println!("{}", elf.len());

    Ok(())
}
