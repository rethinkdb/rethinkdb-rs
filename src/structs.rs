//! Common ReQL data types

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
    __bch: Option<()>,
}

/// Structure of data in `cluster_config` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClusterConfig {
    pub id: String,
    pub heartbeat_timeout_secs: u32,
    __bch: Option<()>,
}
