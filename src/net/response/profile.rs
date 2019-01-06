use serde::Deserialize;

/// Profiling information about the execution of the query
#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    description: Option<String>,
    #[serde(rename = "duration(ms)")]
    duration: Option<f64>,
    sub_tasks: Option<Vec<Profile>>,
    parallel_tasks: Option<Vec<Vec<Profile>>>,
}
