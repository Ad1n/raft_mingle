use error::SimpleResult;

pub(crate) trait Storage {
    fn set(&self, k: String, v: Vec<u8>) -> SimpleResult<bool>;

    fn get(&self, k: String) -> SimpleResult<Vec<u8>>;

    fn has_data(&self) -> SimpleResult<bool>;
}
