use std::collections::HashMap;

pub struct SiteData {
    pub posts: Vec<()>,
    pub projects: Vec<()>,
    pub tags: Vec<()>,
}

pub struct SiteIndex {
    slug_to_posts: HashMap<String, ()>,
    projects: Vec<()>,
    tags: Vec<()>,
}
