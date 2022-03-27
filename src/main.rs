use clap::{Parser, Subcommand};
use wh::WorkHub;
use anyhow::Result;

#[derive(Parser)]
#[clap(
    version = "0.5.4",
    author = "ryoo14 <anana12185@gmail.com",
    about = "Manage working dir",
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    /// Print root dir
    Root{},

    /// Print working dir list
    List{},

    /// Create working dir
    Create{ targetdir_path: String },

    /// Clone Github repository
    Get{ repository: String },
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let wh = WorkHub::new()?;
    
    match &opts.subcmd {
        SubCommand::Root{} => { 
            println!("{}", wh.root);
        },
        SubCommand::List{} => {
            let workdir_list = wh.list()?;
            for workdir in workdir_list {
                println!("{}", workdir);
            }
        }
        SubCommand::Create{ targetdir_path } => {
            match wh.create(targetdir_path) {
                Ok(_) => {
                    println!("Successfully created {}", targetdir_path);
                },
                Err(e) => {
                    println!("{}", e);
                },
            }
        },
        SubCommand::Get{ repository } => {
            match wh.get(repository) {
                Ok(_) => {
                    println!("Successfully cloned {}", repository);
                },
                Err(e) => {
                    println!("{}", e);
                },
            }
        }
    }
    Ok(())
}
