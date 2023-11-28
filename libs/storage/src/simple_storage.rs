use crate::trait_impl;
use error::{CustomError, SimpleResult};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

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
    fn set(&self, k: String, v: Vec<u8>) -> SimpleResult<bool> {
        match self.guard.lock() {
            Ok(guarded_data) => {
                guarded_data.inner.insert(k, v);
                Ok(true)
            },
            Err(err) => return Err(CustomError::new(&err.to_string())),
        }
    }

    fn get(&self, k: String) -> SimpleResult<Vec<u8>> {
        match self.guard.lock() {
            Ok(guarded_data) => match guarded_data.inner.get(k) {},
            Err(err) => return Err(CustomError::new(&err.to_string())),
        }
    }

    fn has_data(&self) -> SimpleResult<bool> {
        match self.guard.lock() {
            Ok(guarded_data) => Ok(!guarded_data.inner.is_empty()),
            Err(err) => return Err(CustomError::new(&err.to_string())),
        }
    }
}
