/// Split a filename's last file extension off, returning both.
///
/// ```rust
/// assert_eq!(split_extension("porn.jpg", ("porn", "jpg")))
/// assert_eq!(split_extension("/foo/bar/spam.jpg", ("/foo/bar/spam", "jpg")))
/// assert_eq!(split_extension("no_extension", ("no_extension", "")))
/// ```
pub fn split_extension(pathname: &str) -> (&str, &str) {
    match pathname.rsplit_once('.') {
        Some((basename, ext)) => (basename, ext),
        None => (pathname, ""),
    }
}
