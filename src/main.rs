use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "ryoo14 <anana12185@gmail.com")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Root(Root),
    List(List),
}

#[derive(Clap)]
// print root directory
struct Root {
}

#[derive(Clap)]
// print working directory list
struct List {
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Root(r) => println!("root"),
        SubCommand::List(l) => println!("list"),
    }
    
}
