#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfo {
     pub success: bool,
     pub min_protocol_version: usize,
     pub max_protocol_version: usize,
     pub server_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    pub protocol_version: i32,
    pub authentication_method: String,
    pub authentication: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
     pub success: bool,
     pub authentication: Option<String>,
     pub error_code: Option<usize>,
     pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthConfirmation {
     pub authentication: String,
}
