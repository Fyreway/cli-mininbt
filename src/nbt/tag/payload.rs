use std::fmt::{self, Write};

use enum_as_inner::EnumAsInner;

use super::{Tag, TagID};

#[derive(Debug, EnumAsInner, PartialEq)]
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
            Self::End => unreachable!(),
        }
    }
}
