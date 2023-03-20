use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
pub enum Format {
    Json,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    pub file: PathBuf,
    #[arg(short, long, value_enum)]
    pub format: Option<Format>,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

pub fn parse() -> Args {
    Args::parse()
}
