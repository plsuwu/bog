use crate::configuration::CONFIG;
use crate::models::{RequestBody, ResponseBody};
use reqwest::Error;
use serenity::model::channel::Message;

/// Constructor for an API request's data.
pub async fn openrouter(msg: Message) -> Result<String, Error> {
    let request_body = crate::template::construct_body(msg);

    // log the built request body to console before sending
    dbg!(&request_body);
    println!("\nsending request, now awaiting inference.");
    let raw_response = post_request(
        "https://openrouter.ai/api/v1/chat/completions",
        &request_body,
    )
    .await?;
    // panics if response cannot be parsed into ResponseBody struct; pretty annoying but
    // fine for now - should be fix this to handle errors with more tact tho (eventually).
    let response_body: ResponseBody = serde_json::from_str(&raw_response).unwrap();

    dbg!(&response_body);
    let message = &response_body.choices[0].message.content;

    Ok(message.to_string())
}

/// Posts a request to the OpenRouter API for LLM inference.
/// - Returns a String to be deserialized with [`serde_json::from_str`] into the [`ResponseBody`] struct.
async fn post_request(url: &str, body: &RequestBody) -> Result<String, Error> {
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .json(body)
        .header(
            "Authorization",
            format!("Bearer {}", &CONFIG.secrets.openrouter_token),
        )
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://plsuwu.com")
        .header("X-Title", "Bog")
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}
