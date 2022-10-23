#![allow(dead_code)]

use args::Args;
use clap::Parser;
use std::{fs, io::Read};

mod args;
mod nbt;

use flate2::read::GzDecoder;
use nbt::tag::Tag;

fn main() {
    let args = Args::parse();
    let bytes = fs::read(&args.file).unwrap();
    let mut gz = GzDecoder::new(bytes.as_slice());
    let mut contents = vec![];
    gz.read_to_end(&mut contents).unwrap();
    let nbt = Tag::new(&mut contents);
    println!("{nbt:#?}");
}
