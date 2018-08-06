//! Common ReQL data types

extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate uuid;
extern crate chrono;

use std::net::IpAddr;
use std::collections::HashMap;
use std::ops::Deref;

use serde_json::Value;
use uuid::Uuid;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

/// Status returned by a write command
#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClusterConfig {
    pub id: String,
    pub heartbeat_timeout_secs: u32,
}

/// Structure of data in `current_issues` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CurrentIssue {
}

/// Structure of data in `db_config` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DbConfig {
}

/// Structure of data in `jobs` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Job {
}

/// Structure of data in `logs` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Log {
}

/// Structure of data in `permissions` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Permission {
}

/// Structure of data in `server_config` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ServerConfig {
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalAddress {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
pub struct Process {
    pub argv: Vec<String>,
    pub cache_size_mb: f64,
    pub pid: u64,
    pub time_started: DateTime,
    pub version: String,
}

/// Structure of data in `server_status` table
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerStatus {
    pub id: Uuid,
    pub name: String,
    pub network: Network,
    pub process: Process,
}

/// Structure of data in `cluster_config` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Stat {
}

/// Structure of data in `cluster_config` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TableConfig {
}

/// Structure of data in `table_status` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TableStatus {
}

/// Structure of data in `uses` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct User {
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Change<O, N> {
    pub old_val: Option<O>,
    pub new_val: Option<N>,
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    pub old_offset: Option<usize>,
    pub new_offset: Option<usize>,
    pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Time {
    #[serde(rename = "$reql_type$")]
    reql_type: String,
    epoch_time: f64,
    timezone: String,
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let time = Time::deserialize(deserializer)?;
        let secs = time.epoch_time.trunc() as i64;
        // RethinkDB timestamps have millisecond precision so we need
        // to convert the milliseconds to nanoseconds first
        let msecs = time.epoch_time.fract().abs() as u32;
        let naive = chrono::NaiveDateTime::from_timestamp(secs, msecs * 1_000_000);
        let dt = chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc);
        Ok(DateTime(dt))
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        unimplemented!();
    }
}

impl Deref for DateTime {
    type Target = chrono::DateTime<chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
