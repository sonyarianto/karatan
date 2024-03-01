use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn json() -> impl Responder {
    let data = serde_json::json!({
        "message": "This is a sample JSON response",
        "timestamp": 1234567890,
        "status": "ok"
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .body(data.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .route("/hello", web::get().to(hello))
            .route("/json", web::get().to(json))
    })
    .bind(("127.0.0.1", 3021))?
    .run()
    .await
}
