use enum_as_inner::EnumAsInner;

use std::string::ToString;

use super::{Tag, TagID};

#[derive(EnumAsInner)]
pub enum TraversedTag<'a> {
    Tag(&'a Tag),
    Contained(&'a Tag),
}

impl TraversedTag<'_> {
    pub fn get_tag(&self) -> &Tag {
        match self {
            Self::Contained(tag) | Self::Tag(tag) => tag,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Path(Vec<TagTraversal>),
    Index(i32),
    Container,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::Path(path) => {
                let str_path: Vec<_> = path.iter().map(ToString::to_string).collect();
                "Invalid path: ".to_string() + &str_path.join(" > ")
            }
            Self::Index(idx) => format!("Invalid index: {idx}"),
            Self::Container => "Cannot traverse into a non-container tag".to_string(),
        }
    }
}

#[derive(Clone, EnumAsInner, Debug)]
pub enum TagTraversal {
    Compound(String),
    Array(i32),
    None,
}

impl ToString for TagTraversal {
    fn to_string(&self) -> String {
        match self {
            Self::Compound(name) => name.clone(),
            Self::Array(idx) => idx.to_string(),
            Self::None => "None".to_string(),
        }
    }
}

impl TagTraversal {
    pub fn traverse<'a>(
        path: &'a [TagTraversal],
        root: &'a Tag,
    ) -> Result<TraversedTag<'a>, Error> {
        // current selected tag
        let mut tag = root;
        // current selected payload
        let mut payload = &tag.payload;
        for traversal in path {
            match traversal {
                TagTraversal::Compound(name) => {
                    let subtags = payload.as_compound().unwrap();
                    tag = subtags
                        .iter()
                        .find(|t| &t.name == name)
                        .ok_or_else(|| Error::Path(path.to_vec()))?;
                    payload = &tag.payload;
                }
                TagTraversal::Array(idx) => {
                    let (tag_id, payloads) = payload.as_list().unwrap();
                    payload = payloads.get(*idx as usize).ok_or(Error::Index(*idx))?;
                    if tag_id != &TagID::Compound || tag_id != &TagID::List {
                        return Err(Error::Container);
                    }
                }
                TagTraversal::None => unreachable!(),
            }
        }

        Ok(match tag.tag_id {
            TagID::Compound
            | TagID::ByteArray
            | TagID::List
            | TagID::IntArray
            | TagID::LongArray => TraversedTag::Tag(tag),
            _ => TraversedTag::Contained(tag),
        })
    }
}
