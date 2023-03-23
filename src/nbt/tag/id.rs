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

    fn is_decimal(self) -> bool {
        matches!(self, Self::Float | Self::Double)
    }

    fn parse_num(self, input: &str) -> Option<TagPayload> {
        if self.is_decimal() {
            let num: f64 = input.parse().ok()?;
            if let Self::Float = self {
                Some(TagPayload::Float({
                    let res = num as f32;
                    if res.is_finite() {
                        Some(res)
                    } else {
                        None
                    }
                }?))
            } else {
                Some(TagPayload::Double(num))
            }
        } else {
            let num: i64 = input.parse().ok()?;
            Some(match self {
                TagID::Byte => TagPayload::Byte(num.try_into().ok()?),
                TagID::Short => TagPayload::Short(num.try_into().ok()?),
                TagID::Int => TagPayload::Int(num.try_into().ok()?),
                TagID::Long => TagPayload::Long(num),
                _ => unreachable!(),
            })
        }
    }

    fn parse_str(input: &str) -> Option<TagPayload> {
        let mut s = String::new();
        let mut escaped = false;
        for (i, ch) in input.strip_prefix('"')?.chars().enumerate() {
            if escaped {
                s.push(match ch {
                    'n' => Some('\n'),
                    't' => Some('\t'),
                    'r' => Some('\r'),
                    '0' => Some('\0'),
                    '\\' | '"' => Some(ch),
                    _ => None,
                }?);
            } else {
                match ch {
                    '\\' => escaped = true,
                    '"' => {
                        return if i == input.len() - 1 {
                            Some(TagPayload::String(s))
                        } else {
                            None
                        }
                    }
                    _ => s.push(ch),
                }
            }
        }

        None
    }

    pub fn parse(self, input: &str) -> Option<TagPayload> {
        match self {
            TagID::Byte
            | TagID::Short
            | TagID::Int
            | TagID::Long
            | TagID::Float
            | TagID::Double => self.parse_num(input),
            TagID::String => Self::parse_str(input),
            _ => None,
        }
    }
}
