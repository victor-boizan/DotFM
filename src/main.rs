use std::env;

use std::path::Path;

use git2::Repository;

mod actions;
mod conf_modules;

fn main() {
    /*default values for variable*/
    let mut pretend = true;
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
                    if args_index != 2 {
                        println!("Unknown parameter: {}", args[args_index]);
                    }
                }
            };
            args_index += 1;
        }
        #[cfg(debug_assertions)]
        println!(
            "\naction: {}\npath: {}\npretend: {}\n",
            action, path, pretend
        );
        match action.as_str() {
            "install" => {
                if args.len() > 2 && conf_modules::is_module(&Path::new(path).join(&args[2])) {
		    /*enter if the second argument is a module*/
                    actions::install::modules(Path::new(path).join(&args[2]), pretend);
                } else {
		    /*if an url is provided but in second argument and their is no "--repo" parameter*/
		    /*add the url to the url variable*/
		    if arg.len() > 2 && &args[2].start_with("https://") && url == "" {
			url = &args[2];
		    }
		    /*if their is an url, it will clone the repo*/
                    if url != "" {
                        let repo = match Repository::clone(url, path) {
                            Ok(repo) => repo,
                            Err(e) => panic!("failed to clone: {}", e),
                        };
                    };
		    /*install all the modules in the DotFM folder*/
                    actions::install::repo(Path::new(path), pretend);
                }
            }
            "uninstall" => {
                println!("uninstall is not implemented yet");
            }
            "init" => {
                println!("init is not implemented yet");
            }
            _ => {
                println!("Unknown action: {}", args[1]);
            }
        }
    } else {
        println!("There is no parameters. The programe can't do anything for now");
    }
}
