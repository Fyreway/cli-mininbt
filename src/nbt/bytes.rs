use std::slice::Iter;

use crate::nbt::tag::TagID;

pub struct NbtBytes<'a> {
    pub bytes: &'a mut Iter<'a, u8>,
}

impl NbtBytes<'_> {
    pub fn next_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut vec = vec![];
        for _ in 0..n {
            vec.push(*self.bytes.next().unwrap());
        }

        vec
    }

    pub fn next_i8(&mut self) -> i8 {
        i8::from_be_bytes([*self.bytes.next().unwrap()])
    }

    pub fn next_i16(&mut self) -> i16 {
        i16::from_be_bytes(self.next_bytes(2).try_into().unwrap())
    }

    pub fn next_u16(&mut self) -> u16 {
        u16::from_be_bytes(self.next_bytes(2).try_into().unwrap())
    }

    pub fn next_i32(&mut self) -> i32 {
        i32::from_be_bytes(self.next_bytes(4).try_into().unwrap())
    }

    pub fn next_i64(&mut self) -> i64 {
        i64::from_be_bytes(self.next_bytes(8).try_into().unwrap())
    }

    pub fn next_f32(&mut self) -> f32 {
        f32::from_be_bytes(self.next_bytes(4).try_into().unwrap())
    }

    pub fn next_f64(&mut self) -> f64 {
        f64::from_be_bytes(self.next_bytes(8).try_into().unwrap())
    }

    pub fn next_str(&mut self) -> String {
        let name_len = self.next_u16();
        if name_len == 0 {
            String::new()
        } else {
            String::from_utf8(self.next_bytes(name_len.into())).unwrap()
        }
    }

    pub fn next_id(&mut self) -> TagID {
        (*self.bytes.next().unwrap()).try_into().unwrap()
    }
}
