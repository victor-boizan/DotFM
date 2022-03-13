use std::env;

use std::path::{Path, PathBuf};

use git2::Repository;
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::os::unix;

fn main() {
    /*default values for variable*/
    let mut pretend = false;
    let mut args_index = 1;
    let path_expand = shellexpand::full("$HOME/.config/dotfm").unwrap();
    let mut path = path_expand.as_ref();
    let mut url = "";
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let action = &args[args_index];
        args_index += 1;

        while args_index < args.len() {
            match args[args_index].to_lowercase().as_str() {
                "--path" => {
                    path = &args[args_index + 1];
                    args_index += 1;
                }
                "--repo" => {
                    repo = &args[args_index + 1];
                    args_index += 1;
                }

                "--pretend" => {
                    pretend = true;
                }
                _ => {
                    println!("Unknown parameter: {}", args[args_index]);
                }
            };
            args_index += 1;
        }
        #[cfg(debug_assertions)]
        println!(
            "\naction: {}\npath: {}\npretend: {}\n",
            action, path, pretend
        );
        if action == "install" {
            if url != "" {
                let repo = match Repository::clone(url, path) {
                    Ok(repo) => repo,
                    Err(e) => panic!("failed to clone: {}", e),
                };
            };
            list_modules(Path::new(path), pretend);
        }
    } else {
        println!("There is no parameters. The programe can't do anything for now");
    }
}

fn list_modules(repo_root: &Path, pretend: bool) -> io::Result<()> {
    if repo_root.is_dir() {
        for entry in fs::read_dir(repo_root)? {
            let entry = entry?;
            let path = entry.path();
            if is_module(&path) {
                println!("{}", path.display());
                install_module(path, pretend);
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

fn install_module(path: PathBuf, pretend: bool) -> std::io::Result<()> {
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
                    if !target.parent().unwrap().exists() {
                        fs::create_dir_all(target.parent().unwrap());
                    }
                    unix::fs::symlink(source, target);
                }
            }
        }
    }
    Ok(())
}
