use crate::simple_storage::StorageResult;

pub(crate) trait Storage {
    fn set(self, k: String, v: Vec<u8>) -> StorageResult<bool>;

    fn get(&self, k: &str) -> Option<&Vec<u8>>;

    fn has_data(&self) -> bool;
}
