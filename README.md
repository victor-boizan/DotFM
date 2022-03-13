# DotFM, a dotfile manager.
## What is it?
This software is an atempt to make my own dotfile manager in rust.
The main idea is to be able to download a binary and execute it to
download and install my configuration file. Also I whant to add somme
modularity to it so each software can be manage separately.

## How it works
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
So you can install existing dotfiles (as long as module are corectly implemented),
or clone and install them.

