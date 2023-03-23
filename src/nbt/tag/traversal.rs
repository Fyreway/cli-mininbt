use enum_as_inner::EnumAsInner;

use std::string::ToString;

use super::{id::TagID, payload::TagPayload, Tag};

#[derive(EnumAsInner)]
pub enum TraversedTag {
    Tag(Tag),
    Payload(TagPayload),
    ContainedTag(Tag),
    ContainedPayload(TagPayload),
}

impl TraversedTag {
    pub fn is_container(&self) -> bool {
        matches!(self, Self::Tag(_) | Self::Payload(_))
    }

    pub fn get_tag(self) -> Option<Tag> {
        match self {
            Self::ContainedTag(t) | Self::Tag(t) => Some(t),
            _ => None,
        }
    }

    pub fn get_payload(self) -> TagPayload {
        match self {
            Self::Payload(p) | Self::ContainedPayload(p) => p,
            _ => self.get_tag().unwrap().payload,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Path(Vec<TagTraversal>),
    Index(i32),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::Path(path) => {
                let str_path: Vec<_> = path.iter().map(ToString::to_string).collect();
                "Invalid path: ".to_string() + &str_path.join(" > ")
            }
            Self::Index(idx) => format!("Invalid index: {idx}"),
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

pub fn traverse(path: &[TagTraversal], root: &Tag) -> Result<TraversedTag, Error> {
    // current selected tag
    let mut tag = Some(root);
    // current selected payload
    let mut payload = &tag.unwrap().payload;
    for traversal in path {
        match traversal {
            TagTraversal::Compound(name) => {
                // Access name in compound
                let subtags = payload.as_compound().unwrap();
                tag = Some(
                    subtags
                        .iter()
                        .find(|t| &t.name == name)
                        .ok_or(Error::Path(path.to_vec()))?,
                );
                payload = &tag.unwrap().payload;
            }
            &TagTraversal::Array(idx) => {
                // Access idx in array
                let (
                        TagPayload::IntArray(payloads)
                        | TagPayload::ByteArray(payloads)
                        | TagPayload::LongArray(payloads)
                        | TagPayload::List(_, payloads)) = payload else {panic!("{payload:?}")};
                payload = payloads.get(idx as usize).ok_or(Error::Index(idx))?;
                tag = None;
            }
            TagTraversal::None => unreachable!(),
        }
    }

    Ok(if let Some(t) = tag {
        if t.is_container() {
            TraversedTag::Tag(t.clone())
        } else {
            TraversedTag::ContainedTag(t.clone())
        }
    } else if Into::<TagID>::into(payload).is_container() {
        TraversedTag::Payload(payload.clone())
    } else {
        TraversedTag::ContainedPayload(payload.clone())
    })
}

pub fn set(path: &[TagTraversal], root: &mut Tag, new: TagPayload) {
    // current selected tag
    let mut tag = Some(root);
    // current selected payload
    let mut payload = &mut tag.unwrap().payload;
    for traversal in path {
        match traversal {
            TagTraversal::Compound(name) => {
                // Access name in compound
                let subtags = payload.as_compound_mut().unwrap();
                tag = Some(subtags.iter_mut().find(|t| &t.name == name).unwrap());
                payload = &mut tag.unwrap().payload;
            }
            &TagTraversal::Array(idx) => {
                // Access idx in array
                let (
                        TagPayload::IntArray(payloads)
                        | TagPayload::ByteArray(payloads)
                        | TagPayload::LongArray(payloads)
                        | TagPayload::List(_, payloads)) = payload else {panic!("{payload:?}")};
                payload = payloads.get_mut(idx as usize).unwrap();
            }
            TagTraversal::None => unreachable!(),
        }
    }

    *payload = new;
}
