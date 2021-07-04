use clap::Clap;
use wh::{root,list};
use anyhow::Result;

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

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let root_path = wh::root::path()?;
    
    match opts.subcmd {
        SubCommand::Root(_) => { 
            println!("{}", root_path);
        },
        SubCommand::List(_) => {
            let workdir_list = wh::list(&root_path)?;
            for workdir in workdir_list {
                println!("{}", workdir);
            }
        }
        SubCommand::Create(c) => println!("create"),
        SubCommand::Delete(d) => println!("delete"),
    }
    Ok(())
}
