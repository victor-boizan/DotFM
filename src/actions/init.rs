use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::fs::File;

use git2::Repository;
pub fn repo(path: &Path, pretend: bool){
    if !path.exists() {
        let mut repo_root = path.to_path_buf();
        /*init the git repo*/
        let mut repo_dir = repo_root.clone();
        if !pretend {
            let repo = match Repository::init(repo_dir) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to init: {}", e),
            };
        }
        /*add a .gitignore*/
        let mut gitignore  = repo_root.clone();
        gitignore.push(".gitignore");
        if !pretend{let mut file = File::create(gitignore);}
        /*add a .gitmodules*/
        let mut gitmodules = repo_root.clone();
        gitmodules.push(".gitmodules");
        if !pretend{let mut file = File::create(gitmodules);}
        /*add a readme*/
        let mut readme = repo_root.clone();
        readme.push("README.md");
        if !pretend{let mut file = File::create(readme);}
    } else {
        println!("The folder already exist. Abort");
    }
}
pub fn module(repo_root: &Path, module_name: &str, pretend: bool){
    let mut module_root = repo_root.to_path_buf();
    module_root.push(module_name);
    if !module_root.exists() {
        /*make the dir*/
        let mut module_dir = module_root.clone();
        if !pretend{fs::create_dir(module_dir);}
        /*add a .module.conf*/
        let mut module_conf = module_root.clone();
        module_conf.push(".module.conf");
        if !pretend{let mut file = File::create(module_conf);}
    } else {
        println!("The folder already exist. Abort");
    }
}
