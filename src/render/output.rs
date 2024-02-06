use std::{fs::create_dir_all, path::Path};

use chrono::Datelike;
use maud::Render;
use tokio::time::Instant;
use tracing::info;
use vfs::{PhysicalFS, VfsPath};

use crate::{media::MediaRegistry, model::site_data::SiteData, templates::Homepage};

#[tracing::instrument(skip_all)]
pub async fn build_static_site(
    content: impl AsRef<Path>,
    templates: impl AsRef<Path>,
    out: impl AsRef<Path>,
) -> anyhow::Result<()> {
    info!(
        out = %out.as_ref().to_string_lossy(),
        content = %content.as_ref().to_string_lossy(),
        templates = %templates.as_ref().to_string_lossy(),
        "Building static site"
    );

    let start = Instant::now();

    create_dir_all(out.as_ref())?;
    let out = VfsPath::new(PhysicalFS::new(out.as_ref()));
    let content = VfsPath::new(PhysicalFS::new(content.as_ref()));
    let templates = VfsPath::new(PhysicalFS::new(templates.as_ref()));

    let media = MediaRegistry::new("/static".into(), out.join("static")?);
    let sd = SiteData::load(content, &media).await?;
    write_static_site(&sd, templates, out)?;

    info!(elapsed = ?start.elapsed(), "Completed");

    Ok(())
}

pub fn write_static_site(
    sd: &SiteData,
    _templates: VfsPath,
    outdir: VfsPath,
) -> anyhow::Result<()> {
    outdir.create_dir_all()?;

    outdir
        .join("index.html")?
        .create_file()?
        .write_all((Homepage {}).render().into_string().as_bytes())?;

    for p in &sd.posts {
        let postdir = outdir
            .join(&p.document.meta.href())
            .unwrap();
        postdir.create_dir_all()?;
        let mut out = postdir.join("index.html").unwrap().create_file()?;
        out.write_all(p.transformed.html.as_bytes())?;
    }

    let projectsdir = outdir.join("projects").unwrap();
    for p in &sd.projects {
        let projectdir = projectsdir.join(&p.document.meta.slug)?;
        projectdir.create_dir_all()?;
        let mut out = projectdir.join("index.html").unwrap().create_file()?;
        out.write_all(p.transformed.html.as_bytes())?;
    }

    Ok(())
}
