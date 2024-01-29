pub struct DateSlug {
    year: u32,
    month: u32,
    day: u32,
    ord: u32,
    name: Option<String>
}

impl DateSlug {
    pub fn new(year: u32, month: u32, day: u32, ord: u32, name: Option<String>) -> DateSlug {
        DateSlug { year, month, day, ord, name }
    }
}

pub enum PartialSlug<'a> {
    Y(u32),
    M(u32, u32),
    D(u32, u32, u32),
    O(u32, u32, u32, u32),
    F(u32, u32, u32, u32, &'a str)
}

