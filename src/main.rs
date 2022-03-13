use std::env;

use std::path::Path;

use git2::Repository;
use std::fs;
use std::io;

mod actions;

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
                    url = &args[args_index + 1];
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
                actions::install::modules(path, pretend);
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
