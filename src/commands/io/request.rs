use super::Request;
use serde::Deserialize;

impl<T: Deserialize + Send + 'static> Request<T> {
    pub fn submit(&self) -> Result<(), ()> {
        Ok(())
    }
}
