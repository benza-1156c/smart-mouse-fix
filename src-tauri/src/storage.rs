use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "benza";
const APPLICATION: &str = "SmartMouseFix";

fn data_dir() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        let dir = proj_dirs.data_dir();
        let _ = fs::create_dir_all(dir);
        dir.to_path_buf()
    } else {
        PathBuf::from(".")
    }
}

/// Get the file path for persisting left-click debounce data.
pub fn left_click_path() -> PathBuf {
    data_dir().join("left_click.json")
}

/// Get the file path for persisting hold-click data (future use).
pub fn hold_click_path() -> PathBuf {
    data_dir().join("hold_click.json")
}

