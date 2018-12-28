use std::path::Path;

impl<'a> Default for super::Connect<'a> {
    fn default() -> Self {
        super::Connect {
            host: [127, 0, 0, 1].into(),
            port: 28015,
            db: "test",
            user: "admin",
            password: "",
            timeout: 20,
            ssl: None,
            multiplex: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ssl<'a> {
    pub ca_certs: &'a Path,
}
