use std::{ffi::OsString, fs::create_dir_all, path::Path};

use maud::Render;
use tokio::time::Instant;
use tracing::{debug, info};
use vfs::{PhysicalFS, VfsError, VfsPath};
use walkdir::WalkDir;

use crate::{
    media::MediaRegistry,
    model::SiteData,
    templates::{
        ArbitraryPageRender, BaseRenderer, BlogIndexPage, Homepage, ProjectIndexPage,
        RenderPost, RenderProject, TagPage,
    },
};

use super::rss::make_rss;

#[tracing::instrument(skip_all)]
pub async fn build_static_site(
    content: impl AsRef<Path>,
    out: impl AsRef<Path>,
    script_assets: Option<impl AsRef<Path>>,
) -> anyhow::Result<()> {
    let script_assets = script_assets.map(|s| s.as_ref().to_owned());
    info!(
        out = %out.as_ref().to_string_lossy(),
        content = %content.as_ref().to_string_lossy(),
        script_assets = ?script_assets.clone().map(|s| s.to_string_lossy().into_owned()),
        "Building static site"
    );

    let script_templates = match &script_assets {
        Some(p) => gather_templates(p)?,
        None => vec![],
    };

    let start = Instant::now();

    create_dir_all(out.as_ref())?;
    let out = VfsPath::new(PhysicalFS::new(out.as_ref()));
    let content = VfsPath::new(PhysicalFS::new(content.as_ref()));

    let media = MediaRegistry::new("/static".into(), out.join("static")?);
    let sd = SiteData::load(content, &media).await?;
    write_static_site(&sd, out, script_templates)?;

    info!(elapsed = ?start.elapsed(), "Completed");

    Ok(())
}

pub fn write_static_site(
    sd: &SiteData,
    outdir: VfsPath,
    script_templates: Vec<String>,
) -> anyhow::Result<()> {
    let index = sd.build_index();

    let renderer = BaseRenderer {
        script_templates,
        site_data: sd,
        site_index: &index,
    };
    outdir.create_dir_all()?;

    write_file(
        &outdir.join("feed.xml")?,
        make_rss("https://astrid.tech", &sd.posts)
            .to_string()
            .as_bytes(),
    )?;

    write_markup(&outdir, renderer.render_page(Homepage))?;

    write_markup(&outdir.join("blog")?, renderer.render_page(BlogIndexPage))?;
    for p in &sd.posts {
        write_markup(
            &outdir.join(&p.document.meta.href())?,
            renderer.render_page(RenderPost::from(p)),
        )?;
    }

    write_markup(
        &outdir.join("projects")?,
        renderer.render_page(ProjectIndexPage {
            projects: sd.projects.iter().collect(),
            tags: &sd.tags,
        }),
    )?;
    for p in &sd.projects {
        write_markup(
            &outdir.join(&p.document.meta.href())?,
            renderer.render_page(RenderProject::from(p)),
        )?;
    }

    for (slug, settings) in &sd.tags {
        write_markup(
            &outdir.join(format!("t/{slug}"))?,
            renderer.render_page(TagPage {
                slug,
                settings,
                posts: index.tag_to_posts[slug.as_str()].clone(),
                projects: index.tag_to_projects[slug.as_str()].clone(),
                all_tags: &sd.tags,
            }),
        )?;
    }

    for p in &sd.pages {
        write_markup(
            &outdir.join(&p.meta().slug)?,
            renderer.render_page(ArbitraryPageRender::from(p)),
        )?;
    }

    outdir.join(".nojekyll")?.create_file()?;

    Ok(())
}

#[tracing::instrument(skip_all, fields(path = path.as_str()))]
fn write_file(path: &VfsPath, r: &[u8]) -> Result<(), VfsError> {
    debug!("writing output file");
    path.parent().create_dir_all()?;
    path.create_file()?.write_all(r)?;

    Ok(())
}

fn write_markup(path: &VfsPath, r: impl Render) -> Result<(), VfsError> {
    let rendered = r.render().into_string();
    let entity_escaped = htmlentity::entity::encode(
        &rendered.as_bytes(),
        &htmlentity::entity::EncodeType::NamedOrHex,
        &htmlentity::entity::CharacterSet::NonASCII,
    );
    write_file(&path.join("index.html")?, &entity_escaped.into_bytes())?;
    Ok(())
}

fn gather_templates(script_assets: impl AsRef<Path>) -> anyhow::Result<Vec<String>> {
    let mut script_templates = vec![];
    for entry in WalkDir::new(script_assets) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if path.extension() == Some(&OsString::from("html")) {
            script_templates.push(std::fs::read_to_string(path)?)
        }
    }
    Ok(script_templates)
}
