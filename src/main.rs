use clap::Parser;

mod cli;
mod loading;
mod metadata;
mod site_data;
mod watch;

fn main() {
    let args = cli::TopLevel::parse();

    match args.command {
        cli::Subcommand::Build(_) => todo!(),
        cli::Subcommand::Watch(_) => todo!(),
    }
}
