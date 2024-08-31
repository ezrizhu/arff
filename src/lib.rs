use axum::{
    routing::get,
    Router,
    extract::Path,
    http::{
        Request,
        HeaderMap,
    },
};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/:name", get(get_object))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn health(req: Request<axum::body::Body>) -> String {
    format!("Arff! Served from {}",
        req.extensions().get::<worker::Cf>().unwrap().colo())
}

pub async fn get_object(Path(name): Path<String>, req: Request<axum::body::Body>) -> (HeaderMap, String) {
    let mut resp_header = HeaderMap::new();
    resp_header.insert("Content-Type", "text/plain".parse().unwrap());
    resp_header.insert("Server", req.extensions().get::<worker::Cf>().unwrap().colo().parse().unwrap());
    (resp_header, String::from(name))
}
