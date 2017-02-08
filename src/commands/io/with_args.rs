use super::Response;
use commands::Args;

impl<T> Response<T> {
    pub fn with_args(&self, _args: Args) -> Response<T> {
        unimplemented!();
    }
}
