use crate::nbt::bytes::NbtBytes;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TagID {
    End,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

impl TryFrom<u8> for TagID {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::End as u8 => Ok(Self::End),
            x if x == Self::Byte as u8 => Ok(Self::Byte),
            x if x == Self::Short as u8 => Ok(Self::Short),
            x if x == Self::Int as u8 => Ok(Self::Int),
            x if x == Self::Long as u8 => Ok(Self::Long),
            x if x == Self::Float as u8 => Ok(Self::Float),
            x if x == Self::Double as u8 => Ok(Self::Double),
            x if x == Self::ByteArray as u8 => Ok(Self::ByteArray),
            x if x == Self::String as u8 => Ok(Self::String),
            x if x == Self::List as u8 => Ok(Self::List),
            x if x == Self::Compound as u8 => Ok(Self::Compound),
            x if x == Self::IntArray as u8 => Ok(Self::IntArray),
            x if x == Self::LongArray as u8 => Ok(Self::LongArray),
            _ => Err(value),
        }
    }
}

#[derive(Debug)]
pub enum TagPayload {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<TagPayload>),
    String(String),
    List(TagID, Vec<TagPayload>),
    Compound(Vec<Tag>),
    IntArray(Vec<TagPayload>),
    LongArray(Vec<TagPayload>),
}

#[derive(Debug)]
pub struct Tag {
    name: String,
    payload: TagPayload,
}

impl Tag {
    /// Consumes and returns every tag up to and including TAG_End.
    fn get_compound(nbt_bytes: &mut NbtBytes) -> Vec<Tag> {
        let mut tags = vec![];

        loop {
            let tag_id = nbt_bytes.next_id();
            let name = if tag_id != TagID::End {
                nbt_bytes.next_str()
            } else {
                String::new()
            };

            tags.push(Tag {
                name,
                payload: Tag::get_payload(nbt_bytes, tag_id),
            });

            if tag_id == TagID::End {
                break;
            }
        }

        tags
    }

    /// Gets the payload from a tag type.
    fn get_payload(nbt_bytes: &mut NbtBytes, tag_id: TagID) -> TagPayload {
        match tag_id {
            TagID::End => TagPayload::End,
            TagID::Byte => TagPayload::Byte(nbt_bytes.next_i8()),
            TagID::Short => TagPayload::Short(nbt_bytes.next_i16()),
            TagID::Int => TagPayload::Int(nbt_bytes.next_i32()),
            TagID::Long => TagPayload::Long(nbt_bytes.next_i64()),
            TagID::Float => TagPayload::Float(nbt_bytes.next_f32()),
            TagID::Double => TagPayload::Double(nbt_bytes.next_f64()),
            TagID::ByteArray => TagPayload::ByteArray(
                (0..nbt_bytes.next_i32())
                    .map(|_| TagPayload::Byte(nbt_bytes.next_i8()))
                    .collect(),
            ),
            TagID::String => TagPayload::String(nbt_bytes.next_str()),
            TagID::List => {
                let id = nbt_bytes.next_id();
                let mut payloads = vec![];
                for _ in 0..nbt_bytes.next_i32() {
                    payloads.push(Tag::get_payload(nbt_bytes, id));
                }
                TagPayload::List(id, payloads)
            }
            TagID::Compound => TagPayload::Compound(Tag::get_compound(nbt_bytes)),
            TagID::IntArray => TagPayload::IntArray(
                (0..nbt_bytes.next_i32())
                    .map(|_| TagPayload::Int(nbt_bytes.next_i32()))
                    .collect(),
            ),
            TagID::LongArray => TagPayload::LongArray(
                (0..nbt_bytes.next_i32())
                    .map(|_| TagPayload::Long(nbt_bytes.next_i64()))
                    .collect(),
            ),
        }
    }

    pub fn new<'a>(bytes: &Vec<u8>) -> Tag {
        let mut nbt_bytes = NbtBytes {
            bytes: &mut bytes.iter(),
        };
        let tag_id = nbt_bytes.next_id();
        let name = if tag_id != TagID::End {
            nbt_bytes.next_str()
        } else {
            String::new()
        };

        Tag {
            name,
            payload: Tag::get_payload(&mut nbt_bytes, tag_id),
        }
    }
}
