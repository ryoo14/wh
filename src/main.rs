use clap::Clap;
use wh::{root, list, create};
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
    target_dir_name: String,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let whroot_path = root::path()?;
    
    match opts.subcmd {
        SubCommand::Root(_) => { 
            println!("{}", whroot_path);
        },
        SubCommand::List(_) => {
            let workdir_list = list(&whroot_path)?;
            for workdir in workdir_list {
                println!("{}", workdir);
            }
        }
        SubCommand::Create(c) => {
            let target_dir_fullpath = whroot_path + "/" + &c.target_dir_name;
            match create(&target_dir_fullpath) {
                Ok(_) => {
                    println!("{}", target_dir_fullpath);
                },
                Err(e) => {
                    println!("{}", e);
                },
            }
        },
    }
    Ok(())
}
