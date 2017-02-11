extern crate scan_dir;

mod config;

use config::Config;
use scan_dir::ScanDir;

fn main() {
    let cfg = Config::new();

    // We need to scan all files and directories in the build dir
    // ignoring symlinks (for security), hidden and backup files
    let mut all = ScanDir::all();
    all.skip_symlinks(true)
        .skip_hidden(true)
        .skip_backup(true);

    // Run the build
    all.walk(&cfg.docs_dir, |iter| {
        let paths = iter.map(|(entry, name)| (entry.path(), name));

        for (path, name) in paths {
            if path.is_file() {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }).unwrap();
}
