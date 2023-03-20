use crate::{args::Format, nbt::tag::Tag};

pub fn get_ext(fmt: &Format) -> String {
    match fmt {
        Format::Json => "json",
    }
    .to_string()
}

pub fn translate(tag: &Tag, fmt: &Format) -> String {
    match fmt {
        Format::Json => translate_json(tag),
    }
}

pub fn translate_json(tag: &Tag) -> String {
    todo!()
}
