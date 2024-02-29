use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use route_handlers::*;
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;

mod route_handlers;

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
