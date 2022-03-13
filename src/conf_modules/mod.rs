use std::path::Path;

use std::fs;
use std::io;

pub fn list_modules(repo_root: &Path, pretend: bool) -> io::Result<()> {
    if repo_root.is_dir() {
        for entry in fs::read_dir(repo_root)? {
            let entry = entry?;
            let path = entry.path();
            if is_module(&path) {
                println!("{}", path.display());
                crate::actions::install::modules(path, pretend);
            } else {
            }
        }
    }
    Ok(())
}

fn is_module(path: &Path) -> bool {
    let mut module_file = path.to_path_buf();
    module_file.push(".module.conf");
    if module_file.is_file() {
        return true;
    }

    return false;
}
