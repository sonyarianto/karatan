use hyper::{Body, Request, Response, StatusCode};
use serde_json::{json, to_string};
use std::convert::Infallible;

pub async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, World!")))
}

pub async fn about(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("This is about page")))
}

pub async fn upload(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    unimplemented!()
}

pub async fn json(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let data = json!({
        "message": "This is a sample JSON response",
        "timestamp": 1234567890,
        "status": "ok"
    });

    let json_str = to_string(&data).unwrap();

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(json_str))
        .unwrap())
}

pub async fn not_found(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap())
}
