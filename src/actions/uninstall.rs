use std::path::{Path, PathBuf};

use regex::Regex;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::os::unix;

pub fn repo(path: &Path, mode: &str, del: bool, pretend: bool){
    for module_path in crate::conf_modules::list_modules(path, pretend) {
        modules(module_path, mode, del, pretend);
    }
}

pub fn modules(path: PathBuf, mode: &str, del: bool, pretend: bool) -> std::io::Result<()> {
    let regex = Regex::new(
        "(?:\\[(?P<header>.*)\\])|(?:\"(?P<source>\\./.*)\"\\s+?=\\s*\"(?P<target>.*)\")",
    )
    .unwrap();
    let module_root = path.clone();
    let mut module_conf = path.clone();
    module_conf.push(".module.conf");
    let config = File::open(module_conf.as_path())?;
    let reader = BufReader::new(config);
    let mut in_file_section = false;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let captures = regex.captures(line.as_str());
        if captures.is_some() {
            let head = captures.as_ref().unwrap().name("header");
            let src = captures.as_ref().unwrap().name("source");
            let targ = captures.as_ref().unwrap().name("target");
            if head.is_some() {
                if head.unwrap().as_str() == "files" {
                    in_file_section = true
                } else {
                    in_file_section = false
                }
            } else if in_file_section {
                let source = module_root
                    .clone()
                    .join(shellexpand::full(src.unwrap().as_str()).unwrap().as_ref())
                    .canonicalize()
                    .unwrap();
                let target_path = shellexpand::full(targ.unwrap().as_str()).unwrap();
                let target = Path::new(target_path.as_ref());

                #[cfg(debug_assertions)]
                println!(
                    "[{}] : source => {} | target => {}",
                    index,
                    source.display(),
                    target.display()
                );
                #[cfg(not(debug_assertions))]
                println!(
                    "\tsource => {} | target => {}",
                    source.display(),
                    target.display()
                );

                if !pretend {
		    match mode {
			"unlink" => {
			    if target.is_symlink() {
				fs::copy(source, target);
			    } 
			}
			"clear" => {
			    if target.is_symlink() {
				fs::remove_file(target);
			    } 
			}
			_ => {}
		    }
                }
            }
        }
    }
    if del {
	fs::remove_dir_all(module_root);
    }
    Ok(())

}
