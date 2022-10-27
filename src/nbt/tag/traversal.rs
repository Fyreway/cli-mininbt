use std::fmt::{self, Write};

use enum_as_inner::EnumAsInner;

use super::{Tag, TagID};

#[derive(Clone, EnumAsInner)]
pub enum TagTraversal {
    Compound(String),
    Array(i32),
    None,
}

impl TagTraversal {
    pub fn traverse<'a>(path: &'a Vec<TagTraversal>, root: &'a Tag) -> Result<&'a Tag, &str> {
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
                        .ok_or("Invalid name")?;
                    payload = &tag.payload;
                }
                TagTraversal::Array(idx) => {
                    let (tag_id, payloads) = payload.as_list().unwrap();
                    payload = payloads.get(*idx as usize).ok_or("Invalid index")?;
                    if tag_id != &TagID::Compound || tag_id != &TagID::List {
                        return Err("Cannot traverse into non-container tag");
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(tag)
    }
}

impl fmt::Display for TagTraversal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compound(name) => f.write_fmt(format_args!("{name}")),
            Self::Array(idx) => f.write_fmt(format_args!("[{idx}]")),
            Self::None => f.write_char('_'),
        }
    }
}
