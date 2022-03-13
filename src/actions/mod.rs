pub mod install;
pub mod uninstall;
pub mod init;

pub enum Action{
    Install,
    Uninstall,
    Init,
}

impl Action {
    pub fn from_str(name: &str) -> Action {
        match name {
            "install"   => {return Action::Install},
            "uninstall" => {return Action::Uninstall},
            "init"      => {return Action::Init},
            _           => {println!("Unknow action: {}", name ); std::process::exit(22)}
        }
    }
}
