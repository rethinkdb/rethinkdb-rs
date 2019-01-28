mod arg;

use crate::Client;

pub use arg::Arg;

impl Client {
    pub fn map<A>(&self, arg: A) -> Client
    where
        A: Into<Arg>,
    {
        Client::new(&self.0, 38, arg.into())
    }
}
