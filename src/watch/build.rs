use std::path::PathBuf;

use notify::{RecursiveMode, Watcher};
use tokio::sync::watch;
use tracing::error;

use crate::{model::site_data::SiteData, render::output::build_static_site};

pub fn reload_on_build(
    content: PathBuf,
    templates: PathBuf,
    out: PathBuf,
    _sdtx: watch::Sender<SiteData>,
) -> anyhow::Result<impl Watcher> {
    let content2 = content.clone();
    let templates2 = out.clone();
    let mut watcher = notify::recommended_watcher(move |ev| match ev {
        Ok(_ev) => {
            let (c, t, o) = (content.clone(), templates.clone(), out.clone());
            tokio::task::spawn_local(async { build_static_site(c, t, o) });
        }
        Err(e) => error!("Received error from notify: {e}"),
    })?;

    watcher.watch(&content2, RecursiveMode::Recursive)?;
    watcher.watch(&templates2, RecursiveMode::Recursive)?;

    Ok(watcher)
}
