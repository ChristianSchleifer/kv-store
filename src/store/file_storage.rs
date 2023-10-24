use std::fs::{File, OpenOptions};
use std::io;
use std::io::{ErrorKind, Seek, SeekFrom, Write};
use std::mem::size_of;
use std::os::unix::fs::FileExt;
use std::path::PathBuf;

use crate::store::entry::Entry;
use crate::store::entry::EntryMetadata;
use crate::store::TOMBSTONE;

pub struct FileStore {
    append_handle: File,
    read_handle: File,
}

impl FileStore {
    pub fn new(db_file: &PathBuf) -> Self {
        let append_handle = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(db_file)
            .unwrap();

        let read_handle = OpenOptions::new()
            .read(true)
            .open(db_file)
            .unwrap();

        Self { append_handle, read_handle }
    }

    pub fn set(&mut self, key: String, value: String) -> io::Result<EntryMetadata> {
        let mut entry = Entry::new(key, value.as_bytes());
        let bytes = entry.serialize();

        self.append_handle.write_all(bytes.as_slice())?;
        let position = self.append_handle.seek(SeekFrom::Current(0))?;

        Ok(EntryMetadata::new(
            position - bytes.len() as u64,
            entry.key.len() as u64,
            entry.value.len() as u64))
    }

    pub fn get(&self, entry_metadata: EntryMetadata) -> io::Result<String> {
        let mut value = vec![0; entry_metadata.value_length as usize];
        self.read_handle.read_exact_at(
            &mut value,
            entry_metadata.start + 2 * (size_of::<u64>() as u64) + entry_metadata.key_length)?;

        let value = String::from_utf8(value);

        match value {
            Ok(s) => Ok(s),
            Err(e) => Err(io::Error::new(ErrorKind::Other, e))
        }
    }

    pub fn delete(&mut self, key: String) -> io::Result<()> {
        let mut entry = Entry::new(key, TOMBSTONE);
        let bytes = entry.serialize();

        self.append_handle.write_all(bytes.as_slice())?;

        Ok(())
    }
}
