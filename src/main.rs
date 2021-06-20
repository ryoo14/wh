use clap::Clap;
use anyhow::Result;
use who;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "ryoo14 <anana12185@gmail.com",
    about = "Manage working dir",
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(about = "Prints root dir")]
    Root(Root),
    #[clap(about = "Prints working dir list")]
    List(List),
    #[clap(about = "Create working dir")]
    Create(Create),
    #[clap(about = "Delete working dir")]
    Delete(Delete),
}

#[derive(Clap)]
// print root directory
struct Root {
}

#[derive(Clap)]
// print working directory list
struct List {
}

#[derive(Clap)]
// create working directory
struct Create {
}

#[derive(Clap)]
// delete working directory
struct Delete {
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Root(_) => { 
            match who::root::path() {
                Ok(p) => {
                    println!("{}", p);
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        },
        SubCommand::List(l) => println!("list"),
        SubCommand::Create(c) => println!("create"),
        SubCommand::Delete(d) => println!("delete"),
    }
}
