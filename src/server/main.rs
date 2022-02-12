use actix_session::{CookieSession, Session};
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    name: String,
    x: i32,
    y: i32,
}

#[get("/{id}/{name}")]
async fn index(params: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = params.into_inner();
    format!("Hello {}! id:{} \n", name, id)
}

#[get("/json/{name}")]
async fn name_json(params: web::Path<String>) -> HttpResponse {
    let name = params.into_inner();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(Point {
            name: format!("{}", name),
            x: 100,
            y: 100,
        })
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::Logger::default())
            .service(name_json)
            .service(index)
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}
