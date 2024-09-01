use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::Request,
    http::StatusCode,
    response::Response,
    routing::{get, post},
    Router,
};
use tower_service::Service;
use worker::*;
mod s3;
mod utils;

fn router(env: Env) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/:name", get(get_object))
        .route("/", post(post_object))
        .route("/", get(home))
        .with_state(env)
}

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
    format!(
        "Arff! Served from {}",
        req.extensions().get::<worker::Cf>().unwrap().colo()
    )
}

pub async fn home() -> String {
    String::from("*barks* x3")
}

#[worker::send]
pub async fn get_object(Path(id): Path<String>, State(env): State<Env>) -> Response {
    let resp = match s3::get(env, id).await {
        Ok(Some(obj)) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(Body::from(obj))
            .unwrap(),
        Ok(None) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body(Body::from("Not Found"))
            .unwrap(),
        Err(error) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(Body::from(
                "Internal Server Error".to_string() + error.to_string().as_str(),
            ))
            .unwrap(),
    };
    resp
}

#[worker::send]
pub async fn post_object(State(env): State<Env>, mut multipart: Multipart) -> Response {
    match multipart.next_field().await {
        Ok(Some(field)) => {
            let data = field.bytes().await.unwrap().to_vec();
            let id = utils::gen_id();

            match s3::put(env, id.clone(), data).await {
                Ok(()) => {
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "text/plain")
                        .body(Body::from(id.clone()))
                        .unwrap()
                }
                Err(err) => {
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "text/plain")
                        .body(Body::from(
                            "Internal Server Error".to_string() + err.to_string().as_str(),
                        ))
                        .unwrap()
                }
            }
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
