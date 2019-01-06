pub(crate) mod message;
pub(crate) mod profile;
pub(crate) mod session;

use std::ops::Deref;

use self::profile::Profile;

/// The response object returned by `query.run()`
#[derive(Debug, Clone)]
pub struct Response<T> {
    value: Vec<T>,
    profile: Vec<Profile>,
}

impl<T> Response<T> {
    pub(crate) fn new(value: Vec<T>, profile: Vec<Profile>) -> Self {
        Self { value, profile }
    }
}

impl<T> Deref for Response<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
