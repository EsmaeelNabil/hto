use indicatif::{ProgressBar, ProgressStyle};
use log::error;
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;
use std::time::Duration;

pub async fn get_api_response(
    prompt: &str,
    model: &str,
    system_message: &str,
    response_mode: &str,
    api_key: &str,
    is_debug: bool,
) -> Result<Value, Box<dyn Error>> {
    if is_debug {
        println!("Prompt: {:?}", prompt);
        println!("Model: {:?}", model);
        println!("System Message: {:?}", system_message);
        println!("Response Mode: {:?}", response_mode);
        println!("API Key: {:?}", api_key);
    }

    let client = Client::new();

    let pb = ProgressBar::new_spinner();
    pb.set_message("Thinking".to_string());
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": model,
            "messages": [
                {"role": "system", "content": system_message},
                {"role": "user", "content": prompt},
            ]
        }))
        .send()
        .await?;

    pb.finish_and_clear();
    if response.status().is_success() {
        let json: Value = response.json().await?;
        if is_debug {
            println!("Response from OpenAI: {:?}", json);
        }
        if response_mode.contains("json") {
            Ok(json)
        } else {
            Ok(json["choices"][0]["message"]["content"].clone())
        }
    } else {
        let error_text = response.text().await?;
        if is_debug {
            error!("Error making request: {}", error_text);
        }
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_text,
        )))
    }
}
