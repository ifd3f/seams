use clap::Parser;
use render::output::build_static_site;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod cli;
mod date_sort;
mod errors;
mod load;
mod media;
mod model;
mod random_coloring;
mod render;
mod templates;
mod transform;
mod upload;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = cli::TopLevel::parse();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive("info".parse().unwrap())
                .from_env_lossy(),
        )
        .init();

    _main(args).await.unwrap();
}

async fn _main(args: cli::TopLevel) -> anyhow::Result<()> {
    match args.command {
        cli::Subcommand::Build(b) => {
            build_static_site(b.src, b.out, b.script_assets).await?;
        }
        cli::Subcommand::Upload(u) => {
            u.run().await?;
        }
        cli::Subcommand::Watch(_) => todo!(),
    }

    Ok(())
}
