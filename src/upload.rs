use s3::{creds::Credentials, Bucket, Region};
use sha2::{Digest, Sha256};
use tracing::{info, debug};

use crate::media::Uploadable;

#[tracing::instrument(skip_all, fields(bucket_name))]
pub async fn upload_to_s3(bucket_name: &str, media: impl Uploadable) -> anyhow::Result<()> {
    let media = media.as_media()?;

    let region = Region::Custom {
        region: "us-west-1".to_string(),
        endpoint: "https://s3.us-west-1.backblazeb2.com".to_string(),
    };
    let credentials = Credentials::default()?;

    let bucket = Bucket::new(bucket_name, region, credentials)?;

    let mut hasher = Sha256::new();
    hasher.update(&media.body);
    let hash = hasher.finalize();
    let b16 = base16::encode_lower(&hash);

    debug!(filename = media.filename, size = media.body.len(), sha = %b16, "Uploadinng");

    let path = match &media.filename {
        Some(n) => format!("{}/{}", b16, n),
        None => b16,
    };
    info!("uploading to {path}");

    let response = bucket
        .put_object_with_content_type(path, &media.body, media.mimetype.unwrap().essence_str())
        .await?;

    info!("{response:?}");

    Ok(())
}
