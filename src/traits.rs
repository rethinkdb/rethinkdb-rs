use super::Result;

pub trait R {
    type Connection;
    fn connect<T: IntoConnectOpts>(&self, opts: T) -> Result<Self::Connection>;
}

pub trait IntoConnectOpts {
    fn into(self) -> ::ConnectOpts;
}

pub trait Connector {
    type Connection;
    fn close(&self, noreply_wait: bool);
    fn reconnect(&self, noreply_wait: bool) -> Self::Connection;
    // use is a reserved keyword in Rust
    fn use_db(&self, db_name: &str) -> Self::Connection;
}
