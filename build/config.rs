use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub docs_dir: PathBuf,
    pub docs_idx: PathBuf,
    pub idx_src: PathBuf,
    pub cmds_src: PathBuf,
    pub version: String,
}

impl Config {
    pub fn new() -> Config {
        let out_dir = env::var("OUT_DIR").unwrap();
        let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let docs_dir = format!("{}/build/docs/api/java", root_dir);
        let docs_idx = format!("{}/index.md", docs_dir);
        let idx_src = format!("{}/index.rs", out_dir);
        let cmds_src = format!("{}/commands.rs", out_dir);
        let version = env::var("CARGO_PKG_VERSION").unwrap();

        Config {
            docs_dir: PathBuf::from(&docs_dir),
            docs_idx: PathBuf::from(&docs_idx),
            idx_src: PathBuf::from(&idx_src),
            cmds_src: PathBuf::from(&cmds_src),
            version: version,
        }
    }
}
