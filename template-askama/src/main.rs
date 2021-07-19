use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use askama_actix::Template;

#[derive(Template)]
#[template(path = "index.j2")]
struct HelloTemplate;

#[derive(Template)]
#[template(path = "page1.j2")]
struct Page1Template {
    pub items: Vec<String>,
}

#[get("/")]
async fn index_handler() -> impl Responder {
    HttpResponse::Ok().body(HelloTemplate {}.render().unwrap())
}

#[get("/page1")]
async fn page1_handler() -> impl Responder {
    let items = vec!["foo".to_string(), "baz".to_string()];
    HttpResponse::Ok().body(Page1Template { items }.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index_handler).service(page1_handler))
        .bind("localhost:3000")?
        .run()
        .await
}
