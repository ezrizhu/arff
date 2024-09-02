use worker::{Error, HttpMetadata};

const BUCKET_NAME: &str = "arff-dev";

pub async fn put(
    env: &worker::Env,
    id: &String,
    data: &Vec<u8>,
    content_type: &String,
) -> Result<(), Error> {
    let bucket = env.bucket(BUCKET_NAME)?;
    let put_builder = bucket.put(id, data.clone());
    put_builder
        .http_metadata(HttpMetadata {
            content_type: Some(content_type.clone()),
            content_language: None,
            content_disposition: None,
            content_encoding: None,
            cache_control: None,
            cache_expiry: None,
        })
        .execute()
        .await?;
    Ok(())
}

pub async fn get(env: &worker::Env, id: &str) -> Result<Option<(Vec<u8>, String)>, Error> {
    let bucket = env.bucket(BUCKET_NAME)?;
    match bucket.get(id).execute().await? {
        Some(obj) => match obj.body() {
            Some(data) => {
                if let Some(content_type) = obj.http_metadata().content_type {
                    return Ok(Some((data.bytes().await?, content_type)))
                } else {
                    return Err(Error::from("metadata not found"))
                }
            }
            None => {
                return Ok(None);
            }
        },
        None => return Ok(None),
    }
}
