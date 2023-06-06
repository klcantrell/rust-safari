use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::net::Ipv4Addr;
use time::{Duration, OffsetDateTime};

#[derive(Serialize)]
struct WeatherForecast {
    #[serde(with = "time::serde::iso8601")]
    date: OffsetDateTime,

    #[serde(rename = "name")]
    temperature_c: i32,

    summary: String,
}

const SUMMARIES: [&str; 10] = [
    "Freezing",
    "Bracing",
    "Chilly",
    "Cool",
    "Mild",
    "Warm",
    "Balmy",
    "Hot",
    "Sweltering",
    "Scorching",
];

#[get("/weatherforecast")]
async fn hello_actix() -> impl Responder {
    HttpResponse::Ok().json(
        (1..5)
            .map(|index| WeatherForecast {
                date: OffsetDateTime::now_utc() + Duration::days(index),
                temperature_c: 32,
                summary: SUMMARIES[make_random(0, SUMMARIES.len() - 1)].to_string(),
            })
            .collect::<Vec<WeatherForecast>>(),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello_actix))
        .bind((Ipv4Addr::LOCALHOST, 3000))?
        .run()
        .await
}

fn make_random(start: usize, end_inclusive: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(start..=end_inclusive)
}
