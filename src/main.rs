mod inventory;
mod crypt;
mod config;
mod ssh;

use std::str::FromStr;
use crate::ssh::do_ssh;
use cron::Schedule;
use chrono::{TimeZone, FixedOffset, Local};
use actix_rt;
use actix_web::*;
use serde_derive::{Deserialize, Serialize};
use core::time::Duration;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use reqwest::{header, Response};


#[derive(Debug, Serialize, Deserialize)]
struct OutgoingWebhook {
    text: String,
}

#[get("/disk_state")]
async fn index(req: HttpRequest) ->  Result<Json<OutgoingWebhook>> {
    let disk_state = do_ssh().await.unwrap_or_else(|e| panic!("ssh failed with {}", e));
    Ok(web::Json(OutgoingWebhook {
        text: disk_state,
    }))
}

#[derive(Debug, Deserialize)]
struct SlashCommandRequest {
    channel_id: String,
    channel_name: String,
    response_url: String,
    team_domain: String,
    team_id: String,
    text: String,
    token: String,
    trigger_id: String,
    user_id: String,
    user_name: String,
}

#[post("/api/slash_command")]
async fn slash_command(req: web::Json<SlashCommandRequest>) -> Result<HttpResponse> {
    // AllowedUntrustedInternalConnections in config.json
    println!("{:?}", req);
    let x = format!("{:?}", req);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .body(x))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    automation_post().await.unwrap();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn automation_post() -> anyhow::Result<()> {
    actix_rt::spawn(async move {
        let expression = "0 0-59/20 9-19 * * 1-5 *";
        let schedule = Schedule::from_str(expression).unwrap();
        let hour = 3600;
        let offset = Some(FixedOffset::east_opt(9 * hour)).unwrap();

        loop {
            let mut upcoming = schedule.upcoming(offset.unwrap()).take(1);
            actix_rt::time::sleep(Duration::from_millis(500)).await;
            let local = &Local::now();

            if let Some(datetime) = upcoming.next() {
                if datetime.timestamp() <= local.timestamp() {

                    let result = post_webhook()
                        .await.unwrap()
                        .text()
                        .await.unwrap();
                    println!("{:?}",result);
                }
            }
        }
    });
    Ok(())
}

async fn post_webhook() -> anyhow::Result<Response> {
    let outgoing_webhook = OutgoingWebhook {
        text: do_ssh().await.unwrap_or_else(|e| panic!("ssh failed with {}", e)),
    };
    let res = post_request(String::from("http://localhost:8065/hooks/ndq8mm17f3rbtfuic197y6jteh"), outgoing_webhook).await;
    res
}

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.100";

pub async fn post_request<T>(url: String, post_data: T) -> anyhow::Result<Response>
    where
        T: serde::Serialize,
{
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json; charset=utf-8"),
    );

    debug_println!("debug: request_url_post {}", url);
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .gzip(false)
        .default_headers(headers)
        .build()?;
    let buf = client
        .post(url.as_str())
        .json(&post_data)
        .send()
        .await
        .expect("error reqwest");
    Ok(buf)
}
