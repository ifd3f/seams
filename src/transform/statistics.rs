pub fn count_swears<'a>(text: &'a str) -> impl Iterator<Item = (&'static str, usize)> + 'a {
    const SWEARS: &[&str] = &[
        "fuck", "shit", "damn", "crap", "piss", "cunt", "cock", "tits",
    ];

    SWEARS
        .into_iter()
        .map(|swear| (*swear, text.matches(swear).count()))
}
