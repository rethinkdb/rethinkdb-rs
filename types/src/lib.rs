//! Common ReQL data types

mod date_time;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::net::IpAddr;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DateTime(OffsetDateTime);

impl From<OffsetDateTime> for DateTime {
    fn from(dt: OffsetDateTime) -> Self {
        Self(dt)
    }
}

impl From<DateTime> for OffsetDateTime {
    fn from(DateTime(dt): DateTime) -> Self {
        dt
    }
}

/// Status returned by a write command
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
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
    pub changes: Option<Vec<Change<Value, Value>>>,
}

/// Structure of data in `cluster_config` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct ClusterConfig {
    pub id: String,
    pub heartbeat_timeout_secs: u32,
}

/// Structure of data in `current_issues` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct CurrentIssue {}

/// Structure of data in `db_config` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct DbConfig {}

/// Structure of data in `jobs` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct Job {}

/// Structure of data in `logs` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct Log {}

/// Structure of data in `permissions` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct Permission {}

/// Structure of data in `server_config` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct ServerConfig {}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct CanonicalAddress {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Network {
    pub canonical_addresses: Vec<CanonicalAddress>,
    pub cluster_port: u16,
    pub connected_to: HashMap<String, bool>,
    pub hostname: String,
    pub http_admin_port: u16,
    pub reql_port: u16,
    pub time_connected: DateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Process {
    pub argv: Vec<String>,
    pub cache_size_mb: f64,
    pub pid: u64,
    pub time_started: DateTime,
    pub version: String,
}

/// Structure of data in `server_status` table
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct ServerStatus {
    pub id: Uuid,
    pub name: String,
    pub network: Network,
    pub process: Process,
}

/// Structure of data in `cluster_config` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct Stat {}

/// Structure of data in `cluster_config` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct TableConfig {}

/// Structure of data in `table_status` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct TableStatus {}

/// Structure of data in `uses` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct User {}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct Change<O, N> {
    pub old_val: Option<O>,
    pub new_val: Option<N>,
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    pub old_offset: Option<usize>,
    pub new_offset: Option<usize>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct ServerInfo {
    pub id: Uuid,
    pub proxy: bool,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
struct BINARY;

#[derive(Debug, Serialize, Deserialize)]
pub struct Binary {
    #[serde(rename = "$reql_type$")]
    reql_type: BINARY,
    pub data: String,
}

impl Binary {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            reql_type: BINARY,
            data: base64::encode(bytes),
        }
    }
}
