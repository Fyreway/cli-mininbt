use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    pub file: PathBuf,
}

pub fn parse() -> Args {
    Args::parse()
}
