use std::path::{Path, PathBuf};
use crate::actions;

pub struct Parameters{
    pub pretend: bool,
    pub action: actions::Action,
    pub path: PathBuf,
    pub del: bool,
    pub url: Option<String>,
    pub mode: Option<actions::uninstall::UninstallMode>,
}
impl Parameters{
    pub fn default(action: actions::Action) -> Self {
        let mut dir_path: Option<PathBuf> = None;
        let path: PathBuf;

        let dotfmdir = shellexpand::full("$DOTFM_DIR") ;
        match dotfmdir {
            Ok(val) => { dir_path = Some([val.as_ref()].iter().collect());}
            Err(val) => {}
        }
        if dir_path.is_none(){

            let xdgconfdir = shellexpand::full("$XDG_CONF_DIR/dotfm");
            match xdgconfdir {
                Ok(val) => {
                    if Path::new(val.as_ref()).exists() {
                        dir_path = Some([val.as_ref()].iter().collect());
                    }
                }
                Err(val) => {}
            }

            if dir_path.is_none() {
                let dotconf = shellexpand::full("$HOME/.config/dotfm");
                match dotconf {
                    Ok(val) => {
                        if Path::new(val.as_ref()).exists() {
                            dir_path = Some([val.as_ref()].iter().collect());
                        }
                    }
                    Err(val) => {
                    }
                }
                if dir_path.is_none(){
                    let home = shellexpand::full("$HOME/dotfm");
                    dir_path = Some([home.unwrap().as_ref()].iter().collect());

                }

            }
        }
        path = dir_path.unwrap();

        Self {
            pretend: false,
            action,
            path,
            url: None,
            mode: None,
            del: false,
        }
    }
}
