use clap::Parser;

mod cli;
mod loading;
mod media;
mod metadata;
mod site_data;
mod transform;
mod watch;

fn main() {
    let args = cli::TopLevel::parse();

    match args.command {
        cli::Subcommand::Build(_) => todo!(),
        cli::Subcommand::Watch(_) => todo!(),
    }
}
