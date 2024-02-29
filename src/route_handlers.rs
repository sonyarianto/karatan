use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;
// use std::future::Future;

pub async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, World!")))
}

pub async fn about(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("This is about page")))
}

pub async fn upload(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    unimplemented!()
}

pub async fn not_found(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap())
}
