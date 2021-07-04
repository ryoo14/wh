# wh

Manage working directories.

## Description

When you want a temporary workspace that you don't want to manage with git, `wh` creates a workspace under a specific dedicated directory.

## Installation

```
cargo install wh
```
## Usage

```
USAGE:
    wh <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    create    Create working dir
    help      Prints this message or the help of the given subcommand(s)
    list      Prints working dir list
    root      Prints root dir
```

## Enviroment Variable

- `WH_ROOT`
  - If this environment variable is set, `wh` will use it as the root path. If it is not set, it defaults to `$HOME/wh`.
