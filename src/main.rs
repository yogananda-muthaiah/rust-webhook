use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::Filter;

#[derive(Deserialize, Serialize, Debug)]
struct WebhookPayload {
    event: String,
    data: HashMap<String, String>,
}

async fn handle_webhook(payload: WebhookPayload) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Received webhook: {:?}", payload);
    Ok(warp::reply::json(&payload))
}

#[tokio::main]
async fn main() {
    // POST /webhook
    let webhook_route = warp::post()
        .and(warp::path("webhook"))
        .and(warp::body::json())
        .and_then(handle_webhook);

    // Start the server
    warp::serve(webhook_route).run(([127, 0, 0, 1], 3030)).await;
}
