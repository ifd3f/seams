use std::path::PathBuf;

use futures::{
    stream::{self, FuturesOrdered},
    StreamExt,
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};
use tracing::{error, info};

use crate::{media::Media, upload::upload_to_s3};

#[derive(clap::Parser)]
pub struct TopLevel {
    #[clap(subcommand)]
    pub command: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    Build(BuildCommand),
    Upload(UploadCommand),
    Watch(WatchCommand),
}

/// Build a static site.
#[derive(clap::Args)]
pub struct BuildCommand {
    /// Content sources directory
    pub src: PathBuf,

    /// Output directory
    #[clap(short, long, default_value = "out")]
    pub out: PathBuf,
}

/// Upload a file to Backblaze.
///
/// Environment variables required: AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY
#[derive(clap::Args)]
pub struct UploadCommand {
    /// The files to upload
    #[clap(required = true)]
    pub src: Vec<PathBuf>,

    /// The bucket to upload to
    #[clap(short, long, default_value = "nyaabucket")]
    pub bucket: String,
}

impl UploadCommand {
    pub async fn run(self) -> anyhow::Result<()> {
        let futures = self
            .src
            .iter()
            .map(|path| async {
                let path = path.clone();
                info!(?path, "uploading file");

                let mut data = vec![];
                let size = BufReader::new(File::open(&path).await?)
                    .read_to_end(&mut data)
                    .await?;
                data.truncate(size);

                let media = Media {
                    filename: Some(path.file_name().unwrap().to_str().unwrap().to_owned()),
                    mimetype: mime_guess::from_path(&path).into_iter().next(),
                    body: data,
                };

                let url = upload_to_s3(&self.bucket, media).await?;

                anyhow::Ok(url)
            })
            .collect::<FuturesOrdered<_>>();

        let mut stream = stream::iter(self.src.iter()).zip(futures);

        let mut fail = false;
        while let Some((path, result)) = stream.next().await {
            match result {
                Ok(url) => println!("{url}"),
                Err(error) => {
                    error!(path = %path.to_string_lossy(), %error, "Error while uploading");
                    fail = true;
                }
            }
        }

        if fail {
            std::process::exit(-1);
        }

        Ok(())
    }
}

/// Serve on a port, while watching and rebuilding the content and templates directories.
///
/// Currently unimplemented.
#[derive(clap::Args)]
pub struct WatchCommand {
    /// Content sources directory
    pub src: PathBuf,

    /// Port to listen on, or 0 to pick a random port
    #[clap(short, long, default_value = "0")]
    pub port: u16,
}
