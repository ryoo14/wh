use anyhow::Result;
use clap::{Parser, Subcommand};
use wh::WorkHub;

#[derive(Parser)]
#[clap(
    version = "0.9.2",
    author = "ryoo14 <anana12185@gmail.com",
    about = "Manage working dir"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    /// Print root dir
    Root {},

    /// Print working dir list
    List {
        #[clap(long, short)]
        full_path: bool,
    },

    /// Create working dir
    Create { targetdir_path: String },

    /// Clone Github repository
    Get { repository: String },
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let wh = WorkHub::new()?;

    match &opts.subcmd {
        SubCommand::Root {} => {
            println!("{}", wh.root);
        }
        SubCommand::List { full_path } => {
            let workdir_list = wh.list()?;
            if *full_path {
                for workdir in workdir_list {
                    println!("{}/{}", WorkHub::new()?.root, workdir);
                }
            } else {
                for workdir in workdir_list {
                    println!("{}", workdir);
                }
            }
        }
        SubCommand::Create { targetdir_path } => match wh.create(targetdir_path) {
            Ok(_) => {
                println!("Successfully created {}", targetdir_path);
            }
            Err(e) => {
                println!("{}", e);
            }
        },
        SubCommand::Get { repository } => match wh.get(repository) {
            Ok(_) => {
                println!("Successfully cloned {}", repository);
            }
            Err(e) => {
                println!("{}", e);
            }
        },
    }
    Ok(())
}
