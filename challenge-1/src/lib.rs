use anyhow::Result;
use serde_json::json;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[http_component]
fn challenge_1(_req: Request) -> Result<Response> {
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(
            json!({
                "message": "Hello, world!"
            })
            .to_string()
            .into(),
        ))?)
}
