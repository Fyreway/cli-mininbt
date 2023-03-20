use crate::nbt::bytes::NbtBytesIter;

use self::{id::TagID, payload::TagPayload};

use super::bytes::ByteResult;

pub mod id;
pub mod payload;
pub mod traversal;

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub tag_id: TagID,
    pub name: String,
    pub payload: TagPayload,
}

impl Tag {
    /// Consumes and returns every tag up to and including `TAG_End`.
    fn get_compound(nbt_bytes: &mut NbtBytesIter) -> ByteResult<Vec<Self>> {
        let mut tags = vec![];

        loop {
            let tag_id = nbt_bytes.next_id()?;
            let name = if tag_id == TagID::End {
                String::new()
            } else {
                nbt_bytes.next_str()?
            };

            tags.push(Self {
                tag_id,
                name,
                payload: Self::get_payload(nbt_bytes, tag_id)?,
            });

            if tag_id == TagID::End {
                break;
            }
        }

        Ok(tags)
    }

    /// Gets the payload to go with a tag type.
    fn get_payload(nbt_bytes: &mut NbtBytesIter, tag_id: TagID) -> ByteResult<TagPayload> {
        Ok(match tag_id {
            TagID::End => TagPayload::End,
            TagID::Byte => TagPayload::Byte(nbt_bytes.next_i8()?),
            TagID::Short => TagPayload::Short(nbt_bytes.next_i16()?),
            TagID::Int => TagPayload::Int(nbt_bytes.next_i32()?),
            TagID::Long => TagPayload::Long(nbt_bytes.next_i64()?),
            TagID::Float => TagPayload::Float(nbt_bytes.next_f32()?),
            TagID::Double => TagPayload::Double(nbt_bytes.next_f64()?),
            TagID::ByteArray => {
                let mut payloads = vec![];
                for _ in 0..nbt_bytes.next_i32()? {
                    payloads.push(TagPayload::Byte(nbt_bytes.next_i8()?));
                }

                TagPayload::ByteArray(payloads)
            }
            TagID::String => TagPayload::String(nbt_bytes.next_str()?),
            TagID::List => {
                let id = nbt_bytes.next_id()?;
                let mut payloads = vec![];
                for _ in 0..nbt_bytes.next_i32()? {
                    payloads.push(Tag::get_payload(nbt_bytes, id)?);
                }
                TagPayload::List(id, payloads)
            }
            TagID::Compound => TagPayload::Compound(Tag::get_compound(nbt_bytes)?),
            TagID::IntArray => {
                let mut payloads = vec![];
                for _ in 0..nbt_bytes.next_i32()? {
                    payloads.push(TagPayload::Int(nbt_bytes.next_i32()?));
                }

                TagPayload::IntArray(payloads)
            }
            TagID::LongArray => {
                let mut payloads = vec![];
                for _ in 0..nbt_bytes.next_i32()? {
                    payloads.push(TagPayload::Long(nbt_bytes.next_i64()?));
                }

                TagPayload::LongArray(payloads)
            }
        })
    }

    pub fn new(bytes: &[u8]) -> ByteResult<Self> {
        let mut nbt_bytes = NbtBytesIter {
            iter: &mut bytes.iter(),
        };
        let tag_id = nbt_bytes.next_id()?;
        let name = if tag_id == TagID::End {
            String::new()
        } else {
            nbt_bytes.next_str()?
        };

        Ok(Self {
            tag_id,
            name,
            payload: Self::get_payload(&mut nbt_bytes, tag_id)?,
        })
    }

    pub fn is_container(&self) -> bool {
        self.tag_id.is_container()
    }
}
