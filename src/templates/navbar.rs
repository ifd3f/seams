use maud::{html, Markup, PreEscaped, Render};

use crate::model::NavbarItem;

#[derive(Debug, Clone)]
pub struct Navbar<'a> {
    pub href: &'a str,
    pub items: &'a [NavbarItem],
    pub navbar_path: &'a [String],
}

#[derive(Debug, Clone)]
pub struct NavbarDropdown<'a> {
    pub href: &'a str,
    pub items: &'a [NavbarItem],
    pub remaining_navbar_path: &'a [String],
    pub nesting: usize,
}

impl Render for Navbar<'_> {
    fn render(&self) -> Markup {
        html! {
            header .site-heading {
                h1 .site-title {
                    a href="/" { "astrid dot tech" }
                }

                nav {
                    ul .navbar {
                        @for item in self.items {
                            li
                                .navitem
                                .active[self.navbar_path.first() == Some(&item.id)]
                            {
                                div .navbar-button {
                                    @match &item.href {
                                        Some(href) => a href=(href) { (PreEscaped(&item.display)) },
                                        None => (PreEscaped(&item.display)),
                                    }
                                }
                                @if !item.children.is_empty() {
                                    (NavbarDropdown {
                                        href: self.href,
                                        items: &item.children,
                                        remaining_navbar_path: &self.navbar_path[1..],
                                        nesting: 1,
                                    })
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Render for NavbarDropdown<'_> {
    fn render(&self) -> Markup {
        html! {
            div .navbar-dropdown {
                ul .navbar {
                    @for item in self.items {
                        li
                            .navitem
                            .active[self.remaining_navbar_path.first() == Some(&item.id)]
                        {
                            div .navbar-button {
                                (PreEscaped(&item.display))
                            }
                            @if !item.children.is_empty() {
                                (NavbarDropdown {
                                    href: self.href,
                                    items: &item.children,
                                    nesting: self.nesting + 1,
                                    remaining_navbar_path: &self.remaining_navbar_path[1..]
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}
