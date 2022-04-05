# DotFM, a dotfile manager.
## What is it?
This software is an attempt to make my dotfile manager in rust.
The main idea is to be able to download a binary and execute it to
download and install my configuration file. Also, I want to add some
modularity to it so each software can be managed separately.

## How it works.
### How to make a module
DotFM uses the file structure in the repository path to detect modules.
Any subfolder of the root of the repo that contains a ".module.conf" file
is a module.

The ".module.conf" explain how to link the files of the module.

First, you need to add '[files]' at the top of your files to start the files section,
anything before is ignored. After you can add lines formatted like this:
"module path" => "destination path"

The module path is relative to the module root and will be the source of the generated link
The destination path would be the full path to the destination link.

The quotation marks are mandatory and you can't use them in the paths.
'~' and environment variable works as a shell would interpret them.

### How to use the software

dotfm action [module name] [action parameters] [global parameters]

#### action
install: install the module (if a module name is given) or the entire repo.
you can clone the repo if you give an URL as the 2nd parameter or if you use --clone



#### Folders
This is the list of folders that will be looked for to find a configuration repository.
- $DOTFM_FOLDER
- $XDG_CONF_DIR/dotfiles
- .config/dotfiles
- ~/.dotfiles
- ~/dotfiles


## The actual state
For now, only the installation has been implemented.
So you can install existing dotfiles (as long as modules are correctly implemented),
or clone and install them.

## What should it be capable of.
This software is still in development. So I will explain what should it be capable to do once it will be finished.

There are three main ideas for this software about its features once finished. They are the installation part, the management part, and the bootstrapping part.

### The (un)installation
Installation and uninstallation should be able to be done for one module at a time or for all the modules in your repository.

- [x] DotFM should be able to install your configurations (that is its 
main purpose). Especially it should be able to install your configuration.

- [ ] Also, for me if something can install stuff on your computer, it's nice if it's also able to uninstall it. That's why uninstall is also one of the core features of this software. For the uninstallation you'll have 2 options: remove the link or replace it with a copy of the file. 

### the management 

- [ ] DotFM should be able to help you edit the configuration. for that, it will be able to move you to a module from anywhere in the system. 

- [ ] You will also be able to open a file in a module via a shortcut.

### bootstapping

- [ ] You should be able with one command to download your configuration and install it directly


