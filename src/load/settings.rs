use frunk::Monoid;
use serde::de::DeserializeOwned;
use tracing::{debug, trace};
use vfs::VfsPath;

use super::util::split_extension;

/// Recursively load all documents in directory.
///
/// This will match files matching this glob:
/// - *.<sub_extension>.yml
#[tracing::instrument(skip_all, fields(path = path.as_str(), ?sub_extension))]
pub fn load_settings_in_dir<S: DeserializeOwned + Monoid>(
    path: VfsPath,
    sub_extension: &str,
) -> anyhow::Result<S> {
    debug!("loading in dir");
    let mut s = S::empty();

    for p in path.walk_dir()? {
        let p = p?;

        let span = tracing::debug_span!("load_settings", path = p.as_str());
        let _enter = span.enter();

        if !p.is_file().unwrap() {
            continue;
        };

        let (rest, ftype) = split_extension(p.as_str());
        let (_, subtype) = split_extension(rest);

        let read: S = match ftype {
            "yml" | "yaml" => {
                if subtype == sub_extension {
                    debug!("loading from file");
                    serde_yaml::from_reader(p.open_file()?)?
                } else {
                    debug!("skipping because it does not have sub_extension {sub_extension}");
                    continue;
                }
            }
            _ => {
                trace!("skipping non-yml");
                continue;
            }
        };

        s = s.combine(&read);
    }

    Ok(s)
}
