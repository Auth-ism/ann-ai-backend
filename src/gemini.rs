use tokio;
use dotenv::dotenv;
use std::env;
use reqwest::Client;
use serde_json::json;

pub fn get_api_key() -> Result<String, String> {
    dotenv().ok();
    // Use underscore instead of hyphen in env var name
    match env::var("GOOGLE_API_KEY") {
        Ok(api_key) => Ok(api_key),
        Err(_) => Err("GEMINI_API environment variable not set".to_string())
    }
}


pub async fn gemini_request(url: String, client: Client) -> Result<(), Box<dyn std::error::Error>> {

    let request_body = json!({ 
        "contents": [
            {
                "parts": [
                    {
                        "text": "Explain how AI works in a few words"
                    }
                ]
            }
        ]
    });

    let response = client.
        post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let response_body = response.text().await?;
    println!("Response: {}", response_body);

    Ok(())
}


pub async fn main() {
    // Properly handle the Result
    let url = match get_api_key() {
        Ok(api_key) => {
            let url = format!(
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
                api_key
            );
            println!("URL: {}", url);
            Some(url)
        },
        Err(error) => {
            eprintln!("Error: {}", error);
            None
        }

        
    };
    let client = Client::new();
    gemini_request(url.unwrap(), client).await.unwrap();

    }

    
