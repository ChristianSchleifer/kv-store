use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use std::path::PathBuf;

use crate::store::entry::EntryMetadata;
use crate::store::TOMBSTONE;

pub struct Index {
    hashmap: HashMap<String, EntryMetadata>,
}

impl Index {
    pub fn new(db_file: &PathBuf) -> Self {
        let mut hashmap = Default::default();

        let mut read_handle = OpenOptions::new()
            .read(true)
            .open(db_file)
            .unwrap();

        read_index(&mut read_handle, &mut hashmap).unwrap();

        Index {
            hashmap,
        }
    }

    pub fn set(&mut self, key: String, entry_metadata: EntryMetadata) {
        self.hashmap.insert(key, entry_metadata);
    }

    pub fn get(&self, key: String) -> Option<EntryMetadata> {
        self.hashmap.get(&key).cloned()
    }

    pub fn delete(&mut self, key: String) {
        self.hashmap.remove(&key);
    }
}

fn read_index(db_file: &mut File, hashmap: &mut HashMap<String, EntryMetadata>) -> io::Result<()> {
    let file_size = db_file.metadata()?.len();
    let mut position;

    loop {
        let mut buffer = [0; 2 * size_of::<u64>()];
        position = db_file.seek(SeekFrom::Current(0))?;

        if position >= file_size {
            break;
        }

        db_file.read_exact(&mut buffer)?;

        let key_length = u64::from_be_bytes(buffer[..size_of::<u64>()].try_into().unwrap());
        let value_length = u64::from_be_bytes(buffer[size_of::<u64>()..].try_into().unwrap());

        let mut key = vec![0; key_length as usize];

        db_file.read_exact(&mut key)?;
        let key = String::from_utf8(key).unwrap();

        if value_length == 1 {
            let mut value = vec![0; 1];
            db_file.read_exact(&mut value)?;
            if value == TOMBSTONE {
                hashmap.remove(&key);
                continue;
            }
        }

        db_file.seek(SeekFrom::Current(value_length as i64))?;

        let entry_metadata = EntryMetadata::new(position, key_length, value_length);
        hashmap.insert(key, entry_metadata);
    }

    Ok(())
}