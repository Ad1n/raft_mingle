use crate::simple_storage::StorageResult;

pub(crate) trait Storage {
    fn set(&self, k: String, v: Vec<u8>) -> StorageResult<bool>;

    fn get(&self, k: String) -> StorageResult<Option<Vec<u8>>>;

    fn has_data(&self) -> StorageResult<bool>;
}
