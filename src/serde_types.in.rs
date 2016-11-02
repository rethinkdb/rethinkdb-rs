use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct ServerInfo {
     success: bool,
     min_protocol_version: usize,
     max_protocol_version: usize,
     server_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    protocol_version: i32,
    authentication_method: String,
    authentication: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthResponse {
     success: bool,
     authentication: Option<String>,
     error_code: Option<usize>,
     error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthConfirmation {
     authentication: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReqlResponse {
    t: i32,
    e: Option<i32>,
    r: Value,
    b: Option<Vec<String>>,
    p: Option<String>,
    n: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WriteStatus {
    pub inserted: u32,
    pub replaced: u32,
    pub unchanged: u32,
    pub skipped: u32,
    pub deleted: u32,
    pub errors: u32,
    pub first_error: Option<String>,
    pub generated_keys: Option<Vec<Uuid>>,
    pub warnings: Option<String>,
    pub changes: Option<Vec<Value>>,
    _p: (),
}
