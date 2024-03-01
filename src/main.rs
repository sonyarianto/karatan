use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// Define not_found handler
async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Page not found")
}

async fn about() -> impl Responder {
    HttpResponse::Ok().body("This is about page")
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

async fn dynamic_route(info: web::Path<(String, u32)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! You are {} years old.", info.0, info.1))
}

async fn connect_to_external_api() -> impl Responder {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/todos")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .route("/hello", web::get().to(about))
            .route("/json", web::get().to(json))
            .route("/{name}/age/{age}", web::get().to(dynamic_route))
            .route("/api/proxy", web::get().to(connect_to_external_api))
            // Handle 404 with custom handler
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 3021))?
    .run()
    .await
}
