use itertools::Itertools;
use rss::{extension::atom::AtomExtensionBuilder, validation::Validate, *};

use crate::{load::document::FullyLoadedDocument, model::metadata::Post};

pub fn make_rss<'a, I>(base_url: &str, posts: I) -> Channel
where
    I: IntoIterator<Item = &'a FullyLoadedDocument<Post>>,
{
    let items = posts
        .into_iter()
        .map(|p| {
            let tags = p.meta().tags.iter();
            let categories = tags
                .map(|t| CategoryBuilder::default().name(t.clone()).build())
                .collect_vec();

            ItemBuilder::default()
                .title(Some(p.meta().title.clone()))
                .link(Some(format!("{}{}", base_url, p.meta().href())))
                .pub_date(Some(p.meta().date.published.to_rfc2822()))
                .content(Some(p.transformed.html.clone()))
                .categories(categories)
                .build()
        })
        .collect_vec();

    let channel = ChannelBuilder::default()
        .title("astrid.tech".to_owned())
        .link(base_url.to_owned())
        .description("website about tech and other shit".to_owned())
        .generator(Some("Seams CMS".to_owned()))
        .language(Some("en".to_owned()))
        .items(items)
        .atom_ext(Some(AtomExtensionBuilder::default().build()))
        .build();

    channel
        .validate()
        .expect("Programmer error while building RSS");

    channel
}
