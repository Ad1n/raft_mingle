use crate::trait_impl;
use error::CustomError;
use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::collections::BTreeMap;
use std::sync::{Arc, OnceLock};
use thiserror::Error;
use tokio::sync::RwLock;

pub static STORAGE: OnceLock<Arc<RwLock<SimpleStorage>>> = OnceLock::new();

pub struct SimpleStorage {
    data: BTreeMap<String, Vec<u8>>,
}

impl SimpleStorage {
    pub fn new() -> Self {
        SimpleStorage {
            data: BTreeMap::new(),
        }
    }

    pub fn serialize_data(&self) -> Result<Vec<u8>, serde_json::Error> {
        //FIXME: Error handling
        serde_json::to_vec(&self.data)
    }

    pub fn deserialize_data(
        &mut self,
        serialized_data: &[u8],
    ) -> Result<(), serde_json::Error> {
        self.data = serde_json::from_slice(serialized_data)?;
        Ok(())
    }
}

impl trait_impl::Storage for SimpleStorage {
    fn set(mut self, key: String, value: Vec<u8>) -> StorageResult<bool> {
        match self.data.entry(key) {
            Occupied(_) => Err(StorageError::Occupied(CustomError::new("Occupied key"))),
            Vacant(entry) => {
                entry.insert(value);
                Ok(true)
            },
        }
    }

    fn get(&self, k: &str) -> Option<&Vec<u8>> {
        self.data.get(k)
    }

    fn has_data(&self) -> bool {
        self.data.is_empty()
    }
}

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Occupied data")]
    Occupied(#[from] CustomError),
}
