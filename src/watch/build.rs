use std::path::Path;

use notify::{RecursiveMode, Watcher};
use tokio::sync::watch;
use tracing::error;

use crate::site_data::SiteData;

pub fn reload_on_build(
    path: impl AsRef<Path>,
    _sdtx: watch::Sender<SiteData>,
) -> anyhow::Result<impl Watcher> {
    let mut watcher = notify::recommended_watcher(|ev| match ev {
        Ok(_ev) => {
            todo!();
        }
        Err(e) => error!("Received error from notify: {e}"),
    })?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    Ok(watcher)
}
