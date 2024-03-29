use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::net::Ipv4Addr;
use time::{
    format_description::{self, FormatItem},
    Duration, OffsetDateTime,
};

#[derive(Serialize)]
struct WeatherForecast {
    date: String,

    #[serde(rename = "temperatureC")]
    temperature_c: i32,

    #[serde(rename = "temperatureF")]
    temperature_f: i32,

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

static DATE_FORMAT: Lazy<Vec<FormatItem>> =
    Lazy::new(|| format_description::parse("[year]-[month]-[day]").unwrap());

#[get("/weatherforecast")]
async fn hello_actix() -> impl Responder {
    let mut rng = thread_rng();

    HttpResponse::Ok().json(
        (1..=5)
            .map(|index| {
                let random_summary_index = rng.gen_range(0..SUMMARIES.len());
                let random_temp_c = rng.gen_range((-20.)..=55.);

                WeatherForecast {
                    date: (OffsetDateTime::now_utc() + Duration::days(index))
                        .format(&DATE_FORMAT)
                        .unwrap(),
                    temperature_c: random_temp_c as i32,
                    temperature_f: 32 + (random_temp_c / 0.5556) as i32,
                    summary: SUMMARIES[random_summary_index].to_string(),
                }
            })
            .collect::<Vec<WeatherForecast>>(),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello_actix))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
