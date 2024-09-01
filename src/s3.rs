use worker::Error;

const BUCKET_NAME: &str = "arff-dev";

pub async fn put(env: worker::Env, id: String, data: Vec<u8>) -> Result<(), Error> {
    let bucket = env.bucket(BUCKET_NAME)?;
    bucket.put(id, data).execute().await?;
    Ok(())
}

pub async fn get(env: worker::Env, id: String) -> Result<Option<Vec<u8>>, Error> {
    let bucket = env.bucket(BUCKET_NAME)?;
    match bucket.get(id).execute().await? {
        Some(obj) => {
            //TODO replace the unwrap
            let data = obj.body().unwrap().bytes().await?;
            return Ok(Some(data));
        }
        None => return Ok(None),
    }
}
