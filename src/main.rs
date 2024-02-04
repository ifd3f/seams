use clap::Parser;
use media::MediaRegistry;
use site_data::SiteData;
use vfs::{PhysicalFS, VfsPath};

mod cli;
mod loading;
mod media;
mod metadata;
mod site_data;
mod transform;
mod watch;

#[tokio::main]
async fn main() {
    let args = cli::TopLevel::parse();

    match args.command {
        cli::Subcommand::Build(b) => {
        }
        cli::Subcommand::Watch(_) => todo!(),
    }
}
