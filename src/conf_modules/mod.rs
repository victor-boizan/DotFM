use std::path::Path;
use std::path::PathBuf;

use std::fs;

pub fn list_modules(repo_root: &Path, pretend: bool) -> Vec<PathBuf> {
    let mut list: Vec<PathBuf> = Vec::new();
    if repo_root.is_dir() {
        for entry in fs::read_dir(repo_root).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if is_module(&path) {
                list.push(path);
            }
        }
    }
    return list;
}
pub fn is_module(path: &Path) -> bool {
    let mut module_file = path.to_path_buf();
    module_file.push(".module.conf");
    if module_file.is_file() {
        return true;
    }

    return false;
}
