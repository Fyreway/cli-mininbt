use std::{fmt::Display, slice::Iter, string::FromUtf8Error};

use crate::nbt::tag::TagID;

pub struct NbtBytes<'a> {
    pub bytes: &'a mut Iter<'a, u8>,
}

pub enum ByteError {
    NextByteError(usize),
    Utf8Error(FromUtf8Error),
    InvalidTagID(u8),
}

impl Display for ByteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NextByteError(n) => f.write_fmt(format_args!("Cannot read {n} bytes ahead")),
            Self::Utf8Error(e) => f.write_fmt(format_args!("UTF8 Error: {e}")),
            Self::InvalidTagID(id) => f.write_fmt(format_args!("Invalid tag ID byte {id:x?}")),
        }
    }
}

pub type ByteResult<T> = Result<T, ByteError>;

impl NbtBytes<'_> {
    pub fn next_bytes(&mut self, n: usize) -> ByteResult<Vec<u8>> {
        let mut vec = vec![];
        for _ in 0..n {
            vec.push(*self.bytes.next().ok_or(ByteError::NextByteError(n))?);
        }

        Ok(vec)
    }

    pub fn next_i8(&mut self) -> ByteResult<i8> {
        Ok(i8::from_be_bytes([*self
            .bytes
            .next()
            .ok_or(ByteError::NextByteError(1))?]))
    }

    pub fn next_i16(&mut self) -> ByteResult<i16> {
        Ok(i16::from_be_bytes(self.next_bytes(2)?.try_into().unwrap()))
    }

    pub fn next_u16(&mut self) -> ByteResult<u16> {
        Ok(u16::from_be_bytes(self.next_bytes(2)?.try_into().unwrap()))
    }

    pub fn next_i32(&mut self) -> ByteResult<i32> {
        Ok(i32::from_be_bytes(self.next_bytes(4)?.try_into().unwrap()))
    }

    pub fn next_i64(&mut self) -> ByteResult<i64> {
        Ok(i64::from_be_bytes(self.next_bytes(8)?.try_into().unwrap()))
    }

    pub fn next_f32(&mut self) -> ByteResult<f32> {
        Ok(f32::from_be_bytes(self.next_bytes(4)?.try_into().unwrap()))
    }

    pub fn next_f64(&mut self) -> ByteResult<f64> {
        Ok(f64::from_be_bytes(self.next_bytes(8)?.try_into().unwrap()))
    }

    pub fn next_str(&mut self) -> ByteResult<String> {
        let name_len = self.next_u16()?;
        if name_len == 0 {
            Ok(String::new())
        } else {
            String::from_utf8(self.next_bytes(name_len.into())?)
                .map_err(|e| ByteError::Utf8Error(e))
        }
    }

    pub fn next_id(&mut self) -> ByteResult<TagID> {
        Ok((*self.bytes.next().ok_or(ByteError::NextByteError(1))?)
            .try_into()
            .map_err(|e| ByteError::InvalidTagID(e))?)
    }
}
