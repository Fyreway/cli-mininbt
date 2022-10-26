use std::fmt::{self, Write};

use enum_as_inner::EnumAsInner;

use crate::nbt::bytes::NbtBytes;

use super::bytes::ByteResult;

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

    /// Gets the enum variant from a u8
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

impl From<&TagPayload> for TagID {
    fn from(payload: &TagPayload) -> Self {
        match payload {
            TagPayload::End => Self::End,
            TagPayload::Byte(_) => Self::Byte,
            TagPayload::Short(_) => Self::Short,
            TagPayload::Int(_) => Self::Int,
            TagPayload::Long(_) => Self::Long,
            TagPayload::Float(_) => Self::Float,
            TagPayload::Double(_) => Self::Double,
            TagPayload::ByteArray(_) => Self::ByteArray,
            TagPayload::String(_) => Self::String,
            TagPayload::List(_, _) => Self::List,
            TagPayload::Compound(_) => Self::Compound,
            TagPayload::IntArray(_) => Self::IntArray,
            TagPayload::LongArray(_) => Self::LongArray,
        }
    }
}

#[derive(Debug, EnumAsInner)]
pub enum TagPayload {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<Self>),
    String(String),
    List(TagID, Vec<Self>),
    Compound(Vec<Tag>),
    IntArray(Vec<Self>),
    LongArray(Vec<Self>),
}

impl fmt::Display for TagPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Byte(byte) => f.write_fmt(format_args!("{byte}b")),
            Self::Short(short) => f.write_fmt(format_args!("{short}s")),
            Self::Int(int) => f.write_str(&int.to_string()),
            Self::Long(long) => f.write_fmt(format_args!("{long}L")),
            Self::Float(float) => f.write_fmt(format_args!("{float}f")),
            Self::Double(double) => f.write_fmt(format_args!("{double}d")),
            Self::ByteArray(bytes) => {
                // Abbreviate if length is greater than 3, otherwise list
                if bytes.len() <= 3 {
                    f.write_str("[B;")?;
                    for b in bytes {
                        if let Self::Byte(byte) = b {
                            f.write_fmt(format_args!("{byte}b,"))?;
                        }
                    }
                    f.write_char(']')
                } else {
                    f.write_str("[B;...]")
                }
            }
            Self::String(string) => f.write_fmt(format_args!("\"{string}\"")),
            Self::List(_, payloads) => {
                // Abbreviate if length is greater than 3, otherwise list
                if payloads.len() <= 3 {
                    f.write_char('[')?;
                    for p in payloads {
                        p.fmt(f)?;
                        f.write_char(',')?;
                    }
                    f.write_char(']')
                } else {
                    f.write_str("[...]")
                }
            }
            Self::Compound(_) => f.write_str("{...}"),
            Self::IntArray(ints) => {
                // Abbreviate if length is greater than 3, otherwise list
                if ints.len() <= 3 {
                    f.write_str("[I;")?;
                    for i in ints {
                        if let Self::Int(int) = i {
                            f.write_fmt(format_args!("{int},"))?;
                        }
                    }
                    f.write_char(']')
                } else {
                    f.write_str("[I;...]")
                }
            }
            Self::LongArray(longs) => {
                // Abbreviate if length is greater than 3, otherwise list
                if longs.len() <= 3 {
                    f.write_str("[L;")?;
                    for l in longs {
                        if let Self::Long(long) = l {
                            f.write_fmt(format_args!("{long}b,"))?;
                        }
                    }
                    f.write_char(']')
                } else {
                    f.write_str("[L;...]")
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Tag {
    pub tag_id: TagID,
    pub name: String,
    pub payload: TagPayload,
}

impl Tag {
    /// Consumes and returns every tag up to and including TAG_End.
    fn get_compound(nbt_bytes: &mut NbtBytes) -> ByteResult<Vec<Self>> {
        let mut tags = vec![];

        loop {
            let tag_id = nbt_bytes.next_id()?;
            let name = if tag_id != TagID::End {
                nbt_bytes.next_str()?
            } else {
                String::new()
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
    fn get_payload(nbt_bytes: &mut NbtBytes, tag_id: TagID) -> ByteResult<TagPayload> {
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

    pub fn new(bytes: &Vec<u8>) -> ByteResult<Self> {
        let mut nbt_bytes = NbtBytes {
            bytes: &mut bytes.iter(),
        };
        let tag_id = nbt_bytes.next_id()?;
        let name = if tag_id != TagID::End {
            nbt_bytes.next_str()?
        } else {
            String::new()
        };

        Ok(Self {
            tag_id,
            name,
            payload: Self::get_payload(&mut nbt_bytes, tag_id)?,
        })
    }
}
