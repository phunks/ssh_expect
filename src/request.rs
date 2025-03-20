
use reqwest::{header, Response};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.100";

pub async fn post_request<T>(url: &str, post_data: &T, insecure: bool) -> anyhow::Result<Response>
where
    T: serde::Serialize,
{
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json; charset=utf-8"),
    );

    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .gzip(false)
        .danger_accept_invalid_certs(insecure)
        .default_headers(headers)
        .build()?;
    let buf = client
        .post(url)
        .json(post_data)
        .send()
        .await
        .expect("error reqwest");
    Ok(buf)
}
