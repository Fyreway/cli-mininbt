#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions
)]

use std::{fs, io::Read};
use ui::UI;
use util::UnwrapOrStrErr;

mod args;
mod nbt;
mod ui;
mod util;

use flate2::read::GzDecoder;
use nbt::tag::Tag;

fn main() {
    let args = args::parse();
    let bytes = fs::read(&args.file).unwrap_or_err("Could not open file");
    let mut gz = GzDecoder::new(bytes.as_slice());
    let mut contents = vec![];
    gz.read_to_end(&mut contents)
        .unwrap_or_err("Could not unzip file");

    let nbt = Tag::new(&contents).unwrap_or_err("Could not parse tag");
    let mut ui = UI::new(&nbt).unwrap_or_err("Could not create UI");
    ui.mainloop().unwrap_or_err("Could not execute mainloop");
}
