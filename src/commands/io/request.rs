use Result;
use super::Request;
use serde::Deserialize;
use futures::{Future, Sink};

impl<T: Deserialize + Send + 'static> Request<T> {
    pub fn submit(&self) -> Result<()> {
        Ok(())
    }
}
