use reqwest::Client;
use serde_json::json;
use std::fs;
use std::env;
use std::path::PathBuf;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Determine the path of the current executable's directory
    let exe_dir = env::current_exe()?
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    // Construct the full path to the webhook_url.txt file
    let webhook_file_path = exe_dir.join("webhook_url.txt");

    // Read the webhook URL from the file
    let webhook_url = fs::read_to_string(webhook_file_path)?
        .trim()
        .to_string();

    // Prompt the user for the message content
    print!("Enter the message to send to the webhook: ");
    io::stdout().flush()?; // Ensure the prompt is shown before input
    let mut message_content = String::new();
    io::stdin().read_line(&mut message_content)?;
    let message_content = message_content.trim(); // Remove any trailing newline characters

    // Create the message payload
    let message = json!({
        "content": message_content
    });

    // Create an HTTP client
    let client = Client::new();

    // Send the POST request
    let response = client
        .post(&webhook_url)
        .json(&message)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Message sent successfully!");
    } else {
        println!("Failed to send message: {:?}", response.status());
    }

    Ok(())
}

