use std::path::PathBuf;

use home;

pub fn get_home_dir() -> Option<PathBuf> {
    return home::home_dir();
}