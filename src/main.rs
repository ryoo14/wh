use clap::Clap;
use wh::WorkHub;
use anyhow::Result;

#[derive(Clap)]
#[clap(
    version = "0.1.1",
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
    targetdir_path: String,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let wh = WorkHub::new()?;
    
    match opts.subcmd {
        SubCommand::Root(_) => { 
            println!("{}", wh.root);
        },
        SubCommand::List(_) => {
            let workdir_list = wh.list()?;
            for workdir in workdir_list {
                println!("{}", workdir);
            }
        }
        SubCommand::Create(c) => {
            match wh.create(&c.targetdir_path) {
                Ok(_) => {
                    println!("Successfully created {}", &c.targetdir_path);
                },
                Err(e) => {
                    println!("{}", e);
                },
            }
        },
    }
    Ok(())
}
