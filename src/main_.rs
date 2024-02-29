// mod route_handlers;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
// use route_handlers::*;
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use serde_json::{json, to_string};

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

fn router(
    req: Request<Body>,
) -> Pin<Box<dyn Future<Output = Result<Response<Body>, Infallible>> + Send + 'static>> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/") => Box::pin(hello(req)),
        (&hyper::Method::GET, "/about") => Box::pin(about(req)),
        (&hyper::Method::POST, "/upload") => Box::pin(upload(req)),
        (&hyper::Method::GET, "/json") => Box::pin(json(req)),
        _ => Box::pin(not_found(req)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3021));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
