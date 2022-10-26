#![allow(dead_code)]

use args::Args;
use clap::Parser;
use std::{fs, io::Read};
use ui::UI;
use util::Unwrap;

mod args;
mod nbt;
mod ui;
mod util;

use flate2::read::GzDecoder;
use nbt::tag::Tag;

fn main() {
    let args = Args::parse();
    let bytes = fs::read(&args.file).unwrap_or_err();
    let mut gz = GzDecoder::new(bytes.as_slice());
    let mut contents = vec![];
    gz.read_to_end(&mut contents).unwrap_or_err();

    let nbt = Tag::new(&contents).unwrap_or_err();
    let mut ui = UI::new(&nbt).unwrap_or_err();
    ui.mainloop().unwrap_or_err();
}
