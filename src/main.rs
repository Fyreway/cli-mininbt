#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions
)]

use std::{fs, io::Read, path::PathBuf};
use translate::{get_ext, translate};
use ui::UI;
use util::UnwrapOrStrErr;

mod args;
mod nbt;
mod translate;
mod ui;
mod util;

use flate2::read::GzDecoder;
use nbt::tag::Tag;

use crate::nbt::encode::encode_tag;

fn main() {
    let args = args::parse();
    let bytes = fs::read(&args.file).unwrap_or_err("Could not open file");
    let mut gz = GzDecoder::new(bytes.as_slice());
    let mut contents = vec![];
    gz.read_to_end(&mut contents)
        .unwrap_or_err("Could not unzip file");

    let nbt = Tag::new(&contents).unwrap_or_err("Could not parse tag");

    if let Some(fmt) = args.format {
        let out = translate(&nbt, &fmt);
        fs::write(
            args.output.unwrap_or({
                let mut p = PathBuf::from(args.file.file_stem().unwrap());
                p.set_extension(get_ext(&fmt));
                p
            }),
            out,
        )
        .unwrap_or_err("Could not write to file");
    } else {
        let mut ui = UI::new(&nbt).unwrap_or_err("Could not create UI");
        ui.mainloop().unwrap_or_err("Could not execute mainloop");
    }
}
