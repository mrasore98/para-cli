# About

`para-cli` is command line tool for managing files using the [PARA](https://fortelabs.com/blog/para/)
system, made popular by Tiago Forte.

It allows users to set up folders for Projects, Areas, Resources, and Archives, then simplifies common workflows
using these folders.

The location of the PARA folders is saved in a configuration file so users can perform file operations
from anywhere on their system without having to reference the paths to the PARA folders directly.

# Example
```
~/Documents/PARA
❯ ls  # Current directory is empty

~/Documents/PARA
❯ para init -n  # Create numbered PARA folders, save the current directory in config

~/Documents/PARA
❯ ls
0_Projects	1_Areas		2_Resources	3_Archives

~/Documents/PARA
❯ para new my_project  # Create a new project

~/Documents/PARA
❯ ls 0_Projects
my_project

~/Documents/PARA
❯ para new -t areas Finances  # Create a new area

~/Documents/PARA
❯ ls 1_Areas
Finances

~/Documents/PARA
❯ para archive 0_Projects/my_project  # Archive from anywhere

~/Documents/PARA
❯ ls 3_Archives
my_project

~/Documents/PARA
❯ ls 0_Projects

```

# Usage

```
para-cli 0.1.0

CLI utility for use with the PARA method.
For more on the PARA method, see https://fortelabs.com/blog/para/

Author: Mathias Alexander <mathias@mathias-alexander.com>

Usage: para [OPTIONS] <COMMAND>

Commands:
  init     Initialize the PARA directories in the current working directory
  new      Create a new folder in one of the PARA folders with a provided name
  archive  Send the files/folders at the provided paths to the Archives
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```
