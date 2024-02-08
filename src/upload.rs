use s3::{creds::Credentials, Bucket, Region};
use sha2::{Digest, Sha256};
use tracing::{debug, debug_span, info};

use crate::media::Uploadable;

#[tracing::instrument(skip_all, fields(bucket_name))]
pub async fn upload_to_s3(bucket_name: &str, media: impl Uploadable) -> anyhow::Result<String> {
    let media = media.as_media()?;
    let endpoint = "https://s3.us-west-000.backblazeb2.com".to_string();

    let region = Region::Custom {
        region: "us-west-1".to_string(),
        endpoint: endpoint.clone(),
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

    let content_type = media.mimetype.map(|m| m.essence_str().to_owned());

    let url = format!("{endpoint}/{bucket_name}/{path}");

    let span = debug_span!("uploading", size = media.body.len(), ?content_type, ?path, %url);
    let _enter = span.enter();

    debug!("performing upload");
    match content_type {
        Some(ct) => {
            bucket
                .put_object_with_content_type(path.clone(), &media.body, &ct)
                .await?
        }
        None => bucket.put_object(path.clone(), &media.body).await?,
    };

    Ok(url)
}
