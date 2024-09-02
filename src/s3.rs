use worker::Error;

const BUCKET_NAME: &str = "arff-dev";

pub async fn put(env: &worker::Env, id: &String, data: &Vec<u8>) -> Result<(), Error> {
    let bucket = env.bucket(BUCKET_NAME)?;
    bucket.put(id, data.clone()).execute().await?;
    Ok(())
}

pub async fn get(env: &worker::Env, id: &String) -> Result<Option<Vec<u8>>, Error> {
    let bucket = env.bucket(BUCKET_NAME)?;
    match bucket.get(id).execute().await? {
        Some(obj) => match obj.body() {
            Some(data) => {
                return Ok(Some(data.bytes().await?));
            }
            None => {
                return Ok(None);
            }
        },
        None => return Ok(None),
    }
}
