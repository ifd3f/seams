use clap::Parser;

mod cli;
mod load;
mod media;
mod model;
mod render;
mod transform;
mod watch;

#[tokio::main]
async fn main() {
    let args = cli::TopLevel::parse();

    match args.command {
        cli::Subcommand::Build(_b) => {}
        cli::Subcommand::Watch(_) => todo!(),
    }
}
