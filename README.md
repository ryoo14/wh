# wh

Manage working directories.

## Description

wh is a tool to manage working directories with or without git management. It is useful to work with interactive filtering tools such as [fzf](https://github.com/junegunn/fzf) and [peco](https://github.com/peco/peco).

Inspired by [ghq](https://github.com/x-motemen/ghq).

## Installation

```
cargo install wh
```

## Usage

```
USAGE:
    wh <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    create    Create working dir
    get       Clone Github repository
    help      Print this message or the help of the given subcommand(s)
    list      Print working dir list
    root      Print root dir
```

The `wh create` command creates the specified directory under the root directory and also creates a `.wh` directory. wh recognizes the directory containing the `.git` or `.wh` directory as a working directory.

## Enviroment Variable

- `WH_ROOT`
  - If this environment variable is set, `wh` will use it as the root path. If it is not set, it defaults to `$HOME/wh`.
