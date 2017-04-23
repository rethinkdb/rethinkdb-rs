//! Common ReQL data types

use std::net::IpAddr;
use std::collections::HashMap;

use DateTime;
use serde_json::Value;
use uuid::Uuid;

/// Status returned by a write command
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
    pub warnings: Option<Vec<String>>,
    pub changes: Option<Value>,
}

/// Structure of data in `cluster_config` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClusterConfig {
    pub id: String,
    pub heartbeat_timeout_secs: u32,
}

/// Structure of data in `current_issues` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentIssue {
}

/// Structure of data in `db_config` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbConfig {
}

/// Structure of data in `jobs` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
}

/// Structure of data in `logs` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
}

/// Structure of data in `permissions` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Permission {
}

/// Structure of data in `server_config` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CanonicalAddress {
    pub host: IpAddr,
    pub port: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Network {
    pub canonical_addresses: Vec<CanonicalAddress>,
    pub cluster_port: u32,
    pub connected_to: HashMap<String, bool>,
    pub hostname: String,
    pub http_admin_port: u32,
    pub reql_port: u32,
    pub time_connected: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Process {
    pub argv: Vec<String>,
    pub cache_size_mb: f64,
    pub pid: u64,
    pub time_started: DateTime,
    pub version: String,
}

/// Structure of data in `server_status` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerStatus {
    pub id: Uuid,
    pub name: String,
    pub network: Network,
    pub process: Process,
}

/// Structure of data in `cluster_config` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stat {
}

/// Structure of data in `cluster_config` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableConfig {
}

/// Structure of data in `table_status` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableStatus {
}

/// Structure of data in `uses` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
}

#[derive(Deserialize, Debug, Clone)]
pub struct Change<O, N> {
    pub old_val: Option<O>,
    pub new_val: Option<N>,
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    pub old_offset: Option<usize>,
    pub new_offset: Option<usize>,
    pub state: Option<String>,
}
