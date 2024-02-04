use chrono::Datelike;
use vfs::VfsPath;

use crate::model::site_data::SiteData;

pub fn write_static_site(sd: &SiteData, outdir: VfsPath) -> anyhow::Result<()> {
    outdir.create_dir_all()?;

    for p in &sd.posts {
        let slugday = p.document.meta.date.published;
        let postdir = outdir
            .join(format!(
                "{}/{:02}/{:02}/{}/{}",
                slugday.year(),
                slugday.month(),
                slugday.day(),
                0usize,
                &p.document.meta.slug
            ))
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
