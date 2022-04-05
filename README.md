# DotFM, a dotfile manager.
## What is it?
This software is an atempt to make my own dotfile manager in rust.
The main idea is to be able to download a binary and execute it to
download and install my configuration file. Also I whant to add somme
modularity to it so each software can be manage separately.

## How it works.
### How to make a module
DotFM use the file structure in the repository path to detect module.
Any subfolder of the root of the repo that contain a ".module.conf" file
is a module.

The ".module.conf" explain how to link the files of the module.

first you need to add '[files]' at the top of your files,
anything before is ignored. After you can add lines formated like this:
"<module file>" => "<destination file>"

the quotation marks are mandatory and you can't use them in the paths.
But '~' and enviroment variable works as a shell would interpret them.

### How to use the software



## The actual state
For now only the instalation of full repositories has been (poorly) implemented
So you can install existing dotfiles (as long as modules are corectly implemented),
or clone and install theme.

## What should it be capable of.
As I wrote it before, this software is not finished so I will explain what should it be capable to do once it will be finished.

There is 3 main idea for this software about it's features once finished. They are the installation part, the management part, and the bootstraping part.

### Installation and uninstallation
Thes tow things should be able to be done for one module at the time or for all the module
in your repository.

- [ ] DotFM should be able to install your configurations (that is actually its 
main purpose). Espescially it should be able to install your personal configuration and mabe 
it will be able to andle system configurations but I'm not sure for now about this. Also mabe
it would be able to install itself if properly configure for your system but this is also 
uncertain.

- [ ] Also, for me if somthing is able to install stuf on your computer, it's really nice if it's also able to uninstall it. Thats why unistall is also one of the core featuresof this 
software.

- [ ] DotFM should be able to help you edit the configuration. for that it will be able to move you to a module from anywere in the ystem.

- [ ] Mabe you will also be able to open a file in a module viaa shortcut for it. for exemple you could try to open kitty/keys.conf.

### Dotfile management
