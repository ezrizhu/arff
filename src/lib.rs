use axum::{
    routing::{get, post},
    Router,
    extract::{Path, State, Multipart},
    http::Request,
    response::Response,
    body::Body,
    http::StatusCode,
};
use rand::Rng;
use tower_service::Service;
use worker::*;

fn router(env: Env) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/:name", get(get_object))
        .route("/", post(post_object))
        .with_state(env)
}

const KV_NS: &str = "dev";

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router(env).call(req).await?)
}

pub async fn health(req: Request<axum::body::Body>) -> String {
    format!("Arff! Served from {}",
        req.extensions().get::<worker::Cf>().unwrap().colo())
}

#[worker::send]
pub async fn get_object(
    Path(name): Path<String>,
    State(env): State<Env>,
) -> Response {
    let kv = env.kv(KV_NS).unwrap();
    let resp = match kv.get(name.as_str()).text().await {
        Ok(Some(obj)) => {
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain")
                .body(Body::from(obj))
                .unwrap()
        }
        Ok(None) => {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "text/plain")
                .body(Body::from("Not Found"))
                .unwrap()
        }
        Err(error) => {
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain")
                .body(Body::from("Internal Server Error".to_string()+error.to_string().as_str()))
                .unwrap()
        }
    };
    resp
}

#[worker::send]
pub async fn post_object(
    State(env): State<Env>,
    mut multipart: Multipart,
) -> Response {
    match multipart.next_field().await {
        Ok(Some(field)) => {
            let data = field.text().await.unwrap();
    let kv = env.kv(KV_NS).unwrap();

    const HEX_CHARS: &[u8] = b"0123456789abcdef";
    let mut rng = rand::thread_rng();
    let id: String = (0..6)
        .map(|_| HEX_CHARS[rng.gen_range(0..16)] as char)
        .collect();

    match kv.put(id.as_str(), data) {
        Ok(opt) => {
            match opt.execute().await {
                Ok(_) => {
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "text/plain")
                        .body(Body::from(id))
                        .unwrap()
                }
                Err(error) => {
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "text/plain")
                        .body(Body::from("Internal Server Error".to_string()+error.to_string().as_str()))
                        .unwrap()
                }
            }
        }
        Err(error) => {
            return Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain")
                .body(Body::from("Internal Server Error".to_string()+error.to_string().as_str()))
                .unwrap()
        }
    };
        }
        Ok(None) => {
            return Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain")
                .body(Body::from("Internal Server Error - none next field"))
                .unwrap()

        }
        Err(_) => {
            return Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain")
                .body(Body::from("Internal Server Error - multipart err"))
                .unwrap()

        }
    };

}
