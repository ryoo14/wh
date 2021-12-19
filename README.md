# wh

Manage working directories.

## Description

It can manage git repository and other working directories.

But now, `list` command only print git repository.  
I'd like to implement the display of directories not managed by git in the future.

## Installation

```
cargo install wh
```

## Usage

```
USAGE:
    wh <SUBCOMMAND>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    create    Create working dir
    get       Clone Github repository
    help      Print this message or the help of the given subcommand(s)
    list      Prints working dir list
    root      Prints root dir
```

## Enviroment Variable

- `WH_ROOT`
  - If this environment variable is set, `wh` will use it as the root path. If it is not set, it defaults to `$HOME/wh`.
