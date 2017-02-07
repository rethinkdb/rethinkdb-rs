use Result;
//use commands::Args;

pub struct Response<T>(T);

/// Create a new connection to the database server
pub trait Connect {
    type ConnectArgs;
    type Connection;

    fn connect(&self, args: Self::ConnectArgs) -> Result<Self::Connection>;
}

/// Run the query
pub trait Run : Connect {
    type RunArgs;

    fn run<T>(&self, args: &Self::RunArgs) -> Response<T>;
}
