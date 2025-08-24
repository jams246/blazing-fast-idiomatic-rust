use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
struct SearchResponse {
    total_count: i32,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("blazing-fast-counter/1.0")
        .build()
        .expect("Failed to build idiomatic client");

    let query = urlencoding::encode(
        r#""blazing fast" OR "idiomatic" language:rust in:readme in:description"#,
    );
    let api_url = format!("https://api.github.com/search/repositories?q={}", query);

    let resp = client
        .get(&api_url)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .expect("Request wasn't blazing fast enough");

    if !resp.status().is_success() {
        panic!(
            "GitHub said no (status {}). Maybe we're too fast?",
            resp.status()
        );
    }

    let body = resp.bytes().await.expect("Failed to read idiomatically");

    let result: SearchResponse =
        serde_json::from_slice(&body).expect("JSON parsing failed (not idiomatic enough?)");

    println!(
        "🔥 Blazingly Fast™ and Idiomatic™ Rust repos: {}",
        result.total_count
    );
}
