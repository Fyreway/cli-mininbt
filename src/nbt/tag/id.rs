use super::payload::TagPayload;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

impl TagID {
    pub fn is_container(self) -> bool {
        matches!(
            self,
            Self::ByteArray | Self::IntArray | Self::List | Self::LongArray | Self::Compound
        )
    }
}
