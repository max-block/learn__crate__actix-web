use actix_web::{get, web::Path, App, HttpServer};

#[get("/")]
async fn index_handler() -> &'static str {
    "it works"
}

#[get("/hello/{name}")]
async fn hello_handler(name: Path<String>) -> String {
    format!("hello, {}", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(|| App::new().service(index_handler).service(hello_handler))
        .bind("localhost:3000")?
        .run()
        .await
}
