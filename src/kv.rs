use worker::{Env, Error};

const KV_NS: &str = "dev";

pub async fn get(env: &Env, id: String) -> Result<String, Error> {
    let kv = env.kv(KV_NS)?;
    kv.get(id.as_str())
        .text()
        .await?
        .ok_or(Error::from("notfound"))
}
