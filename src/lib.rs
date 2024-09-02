use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::header::HeaderMap,
    http::Request,
    http::StatusCode,
    response::Response,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};
mod auth;
mod kv;
mod s3;
mod utils;

const DOMAIN: &str = "arf.sh";
const HOME: &str = "99ec24";

fn router(env: Env) -> Router {
    let origins = [
        "http://arf.sh".parse().unwrap(),
        "https://arf.sh".parse().unwrap(),
    ];

    let layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .expose_headers(Any)
        .allow_origin(origins);

    Router::new()
        .route("/health", get(health))
        .route("/:name", get(get_object))
        .route("/", post(post_object))
        .route("/", get(home))
        .layer(layer)
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

pub async fn home(State(env): State<Env>) -> Response {
    get_object(Path(HOME.to_string()), State(env)).await
}

#[worker::send]
pub async fn get_object(Path(path): Path<String>, State(env): State<Env>) -> Response {
    let id: &str = path.split('.').next().unwrap_or(path.as_str());
    let resp = match s3::get(&env, &id).await {
        Ok(Some((obj, content_type))) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", content_type)
            .body(Body::from(obj))
            .unwrap(),
        Ok(None) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body(Body::from("Not Found"))
            .unwrap(),
        Err(error) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "text/plain")
            .body(Body::from(
                "Internal Server Error".to_string() + error.to_string().as_str(),
            ))
            .unwrap(),
    };
    resp
}

#[worker::send]
pub async fn post_object(
    State(env): State<Env>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    if let Err(()) = auth::check(&headers, &env).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "text/plain")
            .body(Body::from("unauthorized"))
            .unwrap();
    }

    match multipart.next_field().await {
        Ok(Some(field)) => {
            let file_name = field.file_name().unwrap().to_string();
            let file_name_split: Vec<&str> = file_name.split('.').collect();
            let content_type = field.content_type().unwrap().to_string();
            let data = field.bytes().await.unwrap().to_vec();
            let ext = if file_name_split.len() != 2 {
                "".to_owned()
            } else {
                ".".to_owned() + file_name_split[1]
            };

            let id = utils::gen_id();

            match s3::put(&env, &id, &data, &content_type).await {
                Ok(()) => {
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "text/plain")
                        .body(Body::from(format!("{}/{}{}\n", DOMAIN, id.clone(), ext)))
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
