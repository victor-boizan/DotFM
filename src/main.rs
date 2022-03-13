use std::env;

use std::path::{Path,PathBuf};

use git2::Repository;

mod actions;
mod conf_modules;
mod parameters;


fn main() {
    let mut args_index = 1;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let mut param = parameters::Parameters::default(actions::Action::from_str(&args[1]));
        args_index += 1;

        while args_index < args.len() {
            match args[args_index].to_lowercase().as_str() {
                "--path" => {
                    param.path = PathBuf::from(&args[args_index + 1]);
                    args_index += 1;
                }
                "--repo" => {
                    param.url = Some(args[args_index + 1].clone());
                    args_index += 1;
                }
                "--mode" => {
                    param.mode = Some(actions::uninstall::UninstallMode::from_str(&args[args_index + 1]));
                    args_index += 1;
                }
                "--delet" => {
                    param.del = true
                }
                "--pretend" => {
                    param.pretend = true;
                }
                _ => {
                    if args_index != 2 {
                        println!("Unknown parameter: {}", args[args_index]);
                    }
                }
            };
            args_index += 1;
        }
        match param.action {
            actions::Action::Install => {
                if args.len() > 2 && conf_modules::is_module(&param.path.join(&args[2])) {
                    /*enter if the second argument is a module*/
                    actions::install::modules(param.path.join(&args[2]), param.pretend);
                } else {
                    /*if an url is provided but in second argument and their is no "--repo" parameter*/
                    /*add the url to the url variable*/
                    if args.len() > 2 && args[2].starts_with("https://") && param.url.is_none() {
                        param.url = Some(args[2].clone());
                    }
                    /*if their is an url, it will clone the repo*/
                    if param.url.is_some() {
                        let repo = match Repository::clone(param.url.unwrap().as_str(), &param.path) {
                            Ok(repo) => repo,
                            Err(e) => panic!("failed to clone: {}", e),
                        };
                    };
                    /*install all the modules in the DotFM folder*/
                    actions::install::repo(&param.path, param.pretend);
                }
            }
            actions::Action::Uninstall => {
                /*if is a module*/
                if param.mode.is_some() {
                    if args.len() > 2 && conf_modules::is_module(&param.path.join(&args[2])) {
                        actions::uninstall::modules(param.path.join(&args[2]), &param.mode.unwrap(), param.del, param.pretend);
                    } else {
                        /*actions::unistall::repo(Path::new(path), mode, delet, pretend);*/
                    }
                    /*else if is a repo*/
                    /*else -> error*/
                } else {println!("you have to precise what unistallation mode you whant to use"); std::process::exit(1)}
            }
            actions::Action::Init => {
                /*create a new DotFM folder.*/
                if (args.len() == 2) || (args.len() > 2 && args[2].starts_with("--") ){
                    /*initialise a repo*/
                    actions::init::repo(&param.path, param.pretend);
                }else{
                    /*initialise a module*/
                    actions::init::module(&param.path, &args[2], param.pretend);
                }
            }
        }
    } else {
        println!("There is no parameters. The programe can't do anything for now");
    }
}
