use std::{fs::File, io::Read};

use clap::Parser;
use media::Media;
use render::output::build_static_site;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use upload::upload_to_s3;

mod cli;
mod date_sort;
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
            build_static_site(b.src, b.out).await?;
        }
        cli::Subcommand::Upload(u) => {
            let mut data = vec![];
            let size = File::open(&u.src)?.read_to_end(&mut data)?;
            data.truncate(size);

            let media = Media {
                filename: Some(u.src.file_name().unwrap().to_str().unwrap().to_owned()),
                mimetype: mime_guess::from_path(&u.src).into_iter().next(),
                body: data,
            };

            let url = upload_to_s3(&u.bucket, media).await?;
            println!("{url}");
        }
        cli::Subcommand::Watch(_) => todo!(),
    }

    Ok(())
}
