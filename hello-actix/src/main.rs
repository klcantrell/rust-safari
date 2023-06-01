use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::net::Ipv4Addr;

#[derive(Serialize)]
struct HelloResponse {
    greeting: String,
}

#[get("/api/hello-actix")]
async fn hello_actix() -> impl Responder {
    HttpResponse::Ok().json(HelloResponse { greeting: "Hello from Actix on a VM!".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello_actix)
    })
    .bind((Ipv4Addr::LOCALHOST, 3000))?
    .run()
    .await
}
