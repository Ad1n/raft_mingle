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
