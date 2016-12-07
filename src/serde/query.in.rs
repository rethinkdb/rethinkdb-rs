use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct ReqlResponse {
    t: i32,
    e: Option<i32>,
    r: Value,
    b: Option<Value>,
    p: Option<Value>,
    n: Option<Value>,
}

/// Status returned by a write command
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WriteStatus {
    inserted: u32,
    replaced: u32,
    unchanged: u32,
    skipped: u32,
    deleted: u32,
    errors: u32,
    first_error: Option<String>,
    generated_keys: Option<Vec<Uuid>>,
    warnings: Option<Vec<String>>,
    changes: Option<Value>,
}
