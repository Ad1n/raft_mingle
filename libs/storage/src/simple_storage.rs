use crate::trait_impl;
use error::CustomError;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;

pub struct SimpleStorage {
    guard: Arc<Mutex<Data>>,
}

pub struct Data {
    inner: BTreeMap<String, Vec<u8>>,
}

impl SimpleStorage {
    pub fn new() -> Self {
        SimpleStorage {
            guard: Arc::new(Mutex::new(Data {
                inner: BTreeMap::new(),
            })),
        }
    }
}

impl trait_impl::Storage for SimpleStorage {
    fn set(&self, k: String, v: Vec<u8>) -> StorageResult<bool> {
        match self.guard.lock() {
            Ok(mut guarded_data) => {
                guarded_data.inner.insert(k, v);
                Ok(true)
            },
            Err(err) => Err(CustomError::new(&err.to_string()).into()),
        }
    }

    fn get(&self, k: String) -> StorageResult<Option<Vec<u8>>> {
        match self.guard.lock() {
            Ok(guarded_data) => Ok(guarded_data.inner.get(&k).cloned()),
            Err(err) => Err(CustomError::new(&err.to_string()).into()),
        }
    }

    fn has_data(&self) -> StorageResult<bool> {
        match self.guard.lock() {
            Ok(guarded_data) => Ok(!guarded_data.inner.is_empty()),
            Err(err) => Err(CustomError::new(&err.to_string()).into()),
        }
    }
}

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Poisoned mutex")]
    Poisoned(#[from] CustomError),
}
