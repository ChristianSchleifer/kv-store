use std::mem::size_of;

#[derive(Debug, Clone, Copy)]
pub struct EntryMetadata {
    pub start: u64,
    pub key_length: u64,
    pub value_length: u64,
}

impl EntryMetadata {
    pub fn new(start: u64, key_length: u64, value_length: u64) -> Self {
        Self { start, key_length, value_length }
    }
}

pub struct Entry<'a> {
    pub key: String,
    pub value: &'a [u8],
}

impl<'a> Entry<'a> {
    pub fn new(key: String, value: &'a [u8]) -> Entry<'a> {
        Self { key, value }
    }

    pub fn serialize(&mut self) -> Vec<u8> {
        let key_len = self.key.len() as i64;
        let value_len = self.value.len() as i64;
        let mut buff = Vec::with_capacity((key_len + value_len + (3 * size_of::<u64>()) as i64) as usize);

        buff.extend_from_slice(&key_len.to_be_bytes());
        buff.extend_from_slice(&value_len.to_be_bytes());
        buff.extend_from_slice(&self.key.as_bytes());
        buff.extend_from_slice(&self.value);

        buff
    }
}
