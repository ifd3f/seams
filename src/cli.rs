use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct TopLevel {
    #[clap(subcommand)]
    pub command: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    Build(BuildCommand),
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
