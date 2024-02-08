use std::{fs::create_dir_all, path::Path};

use maud::Render;
use tokio::time::Instant;
use tracing::{debug, info};
use vfs::{PhysicalFS, VfsError, VfsPath};

use crate::{
    media::MediaRegistry,
    model::site_data::{SiteData, SiteIndex},
    templates::{BlogIndexPage, Homepage, ProjectIndexPage, RenderPost, RenderProject, TagPage},
};

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
    let index = sd.build_index();

    outdir.create_dir_all()?;

    write_markup(&outdir, Homepage {})?;

    write_markup(
        &outdir.join("blog")?,
        BlogIndexPage {
            posts: sd.posts.iter().collect(),
            tags: &sd.tags,
        },
    )?;
    for p in &sd.posts {
        write_markup(
            &outdir.join(&p.document.meta.href())?,
            RenderPost::from(p).full_content_page(&sd.tags),
        )?;
    }

    write_markup(
        &outdir.join("projects")?,
        ProjectIndexPage {
            projects: sd.projects.iter().collect(),
            tags: &sd.tags,
        },
    )?;
    for p in &sd.projects {
        write_markup(
            &outdir.join(&p.document.meta.href())?,
            RenderProject::from(p).full_content_page(&sd.tags),
        )?;
    }

    for (slug, settings) in &sd.tags {
        write_markup(
            &outdir.join(format!("t/{slug}"))?,
            TagPage {
                slug,
                settings,
                posts: index.tag_to_posts[slug.as_str()].clone(),
                projects: index.tag_to_projects[slug.as_str()].clone(),
                all_tags: &sd.tags,
            },
        )?;
    }

    Ok(())
}

#[tracing::instrument(skip_all, fields(path = path.as_str()))]
fn write_markup(path: &VfsPath, r: impl Render) -> Result<(), VfsError> {
    debug!("writing output file");
    path.create_dir_all()?;
    path.join("index.html")?
        .create_file()?
        .write_all(r.render().into_string().as_bytes())?;

    Ok(())
}
