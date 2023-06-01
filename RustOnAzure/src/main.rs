use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;
use serde::Serialize;
use std::net::Ipv4Addr;

#[derive(Serialize)]
struct HelloResponse {
    greeting: String,
}

#[get("/api/hello-rust-on-azure")]
async fn hello_rust_on_azure() -> impl Responder {
    HttpResponse::Ok().json(HelloResponse { greeting: "Hello from Rust on Azure!".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    HttpServer::new(|| {
        App::new()
            .service(hello_rust_on_azure)
    })
    .bind((Ipv4Addr::LOCALHOST, port))?
    .run()
    .await
}
