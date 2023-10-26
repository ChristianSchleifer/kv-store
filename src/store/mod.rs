use std::io;
use std::path::PathBuf;

use crate::store::file_storage::FileStore;
use crate::store::index::Index;

mod file_storage;
mod index;
mod entry;

pub(crate) struct StorageEngine {
    index: Index,
    file_store: FileStore,
}

impl StorageEngine {
    pub(crate) fn new(path: &PathBuf) -> Self {
        let file_store = FileStore::new(path);

        let index = Index::new(path);

        Self {
            file_store,
            index,
        }
    }

    pub(crate) fn set(&mut self, key: String, value: String) -> io::Result<()> {
        let entry_metadata = self.file_store.set(key.clone(), value)?;
        self.index.set(key, entry_metadata);
        Ok(())
    }

    pub(crate) fn get(&mut self, key: String) -> io::Result<Option<String>> {
        let entry_metadata = self.index.get(key);
        match entry_metadata {
            Some(s) => {
                let value = self.file_store.get(s)?;
                Ok(Some(value))
            }
            None => Ok(None)
        }
    }

    pub(crate) fn delete(&mut self, key: String) -> io::Result<()> {
        self.file_store.delete(key.clone())?;
        self.index.delete(key);
        Ok(())
    }
}

impl Default for StorageEngine {
    fn default() -> Self {
        Self::new(&PathBuf::from("/tmp/log.db"))
    }
}

const TOMBSTONE: &[u8] = &[0x0];
