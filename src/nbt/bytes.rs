use std::{slice::Iter, string::FromUtf8Error};

use crate::nbt::tag::id::TagID;

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum ByteError {
    NextByteError(usize),
    Utf8Error(FromUtf8Error),
    InvalidTagID(u8),
}

impl ToString for ByteError {
    fn to_string(&self) -> String {
        match self {
            Self::NextByteError(n) => format!("Cannot read {n} bytes ahead"),
            Self::Utf8Error(e) => format!("UTF8 Error: {e}"),
            Self::InvalidTagID(id) => format!("Invalid tag ID byte {id:x?}"),
        }
    }
}

pub type ByteResult<T> = Result<T, ByteError>;

/// A wrapper around a u8 iterator that provides functions to read bytes and turn them into data.
pub struct NbtBytesIter<'a> {
    pub iter: &'a mut Iter<'a, u8>,
}

impl NbtBytesIter<'_> {
    /// Reads and consumes a specified number of bytes from the iterator. This
    /// function returns a vector of u8 on success, or a `ByteError::NextByteError`
    /// on failure (if the iterator reaches its end before all bytes were read).
    pub fn next_bytes(&mut self, n: usize) -> ByteResult<Vec<u8>> {
        let mut vec = vec![];
        for _ in 0..n {
            vec.push(*self.iter.next().ok_or(ByteError::NextByteError(n))?);
        }

        Ok(vec)
    }

    /// Takes the next byte and constructs an i8.
    pub fn next_i8(&mut self) -> ByteResult<i8> {
        Ok(i8::from_be_bytes([*self
            .iter
            .next()
            .ok_or(ByteError::NextByteError(1))?]))
    }

    /// Takes the next 2 bytes and constructs a big-endian i16.
    pub fn next_i16(&mut self) -> ByteResult<i16> {
        Ok(i16::from_be_bytes(self.next_bytes(2)?.try_into().unwrap()))
    }

    /// Takes the next 2 bytes and constructs a big-endian u16.
    pub fn next_u16(&mut self) -> ByteResult<u16> {
        Ok(u16::from_be_bytes(self.next_bytes(2)?.try_into().unwrap()))
    }

    /// Takes the next 4 bytes and constructs a big-endian i32.
    pub fn next_i32(&mut self) -> ByteResult<i32> {
        Ok(i32::from_be_bytes(self.next_bytes(4)?.try_into().unwrap()))
    }

    /// Takes the next 8 bytes and constructs a big-endian i64.
    pub fn next_i64(&mut self) -> ByteResult<i64> {
        Ok(i64::from_be_bytes(self.next_bytes(8)?.try_into().unwrap()))
    }

    /// Takes the next 4 bytes and constructs a big-endian f32.
    pub fn next_f32(&mut self) -> ByteResult<f32> {
        Ok(f32::from_be_bytes(self.next_bytes(4)?.try_into().unwrap()))
    }

    /// Takes the next 8 bytes and constructs a big-endian f64.
    pub fn next_f64(&mut self) -> ByteResult<f64> {
        Ok(f64::from_be_bytes(self.next_bytes(8)?.try_into().unwrap()))
    }

    /// Gets the next bytes that represent a string. The way this is done is it
    /// reads a 2-byte big-endian u16 that represents the number of bytes the
    /// string contains, in UTF-8 format.
    pub fn next_str(&mut self) -> ByteResult<String> {
        let name_len = self.next_u16()?;
        if name_len == 0 {
            Ok(String::new())
        } else {
            String::from_utf8(self.next_bytes(name_len.into())?).map_err(ByteError::Utf8Error)
        }
    }

    /// Gets the next bytes and transforms it into a `TagID`. Since there are more
    /// possible u8 values than `TagID` values, this returns the `TagID` on success
    /// and a `ByteError::InvalidTagID` on failure.
    pub fn next_id(&mut self) -> ByteResult<TagID> {
        (*self.iter.next().ok_or(ByteError::NextByteError(1))?)
            .try_into()
            .map_err(ByteError::InvalidTagID)
    }
}
