use clap::{Parser, Subcommand};

mod host;
mod cmd;
mod join;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Host,
    Join{
        #[clap(help = "code of the game to join")]
        code: String,
    }
}

fn make_url(endpoint: &String) -> String {
    "https://rust.nv7haven.com/crufst/".to_string() + endpoint
}

fn main() {
    let args = Args::parse();
    match args.commands {
        Commands::Host => host::host(),
        Commands::Join{code} => {join::join(code)}
    }
}
