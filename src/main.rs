use reqwest;
// use std::io;
use anyhow::anyhow; // Correctly import the anyhow function

use serde_json::json;
// use serde_json::Value;
use anyhow::Result; // Make sure to include anyhow in your Cargo.toml
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MessagesContainer {
    messages: Vec<Message>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // Load .env file variables into the environment

    println!("Welcome to ChatGPT Console!");

    loop {
        let mut input = String::new();

        // Read a line of input from the user
        std::io::stdin().read_line(&mut input)?;

        // Trim the input to remove newline characters
        let input = input.trim();

        // Check for the exit command
        if input == "/exit" {
            println!("Goodbye!");
            break;
        }

        // Example condition to call query_chatgpt()
        if input.starts_with('/') {
            // Extract the query from the input
            let query = &input[1..];

            // Call query_chatgpt() and await its result
            match query_chatgpt(query).await {
                Ok(response) => println!("Response: {}", response),
                Err(e) => eprintln!("Error: {}", e),
            }
        } else {
            println!("You typed: {}", input);
        }
    }

    Ok(())
}


async fn query_chatgpt(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let messages = MessagesContainer {
        messages: vec![
            Message {
                role: "user".to_owned(),
                content: query.to_owned(), // Convert &str to String
            },
        ],
    };
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");
    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    let api_key = api_key; 

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-3.5-turbo",
            // "prompt": format!("You: /{}\n", query),
            "max_tokens": 50,
            "messages": messages.messages // Directly pass the array here
        }))
        .send()
        .await?
        .text()
        .await?;


    let chat_response: serde_json::Value = serde_json::from_str(&response)?;

    if let Some(choices) = chat_response["choices"].as_array() {
        if let Some(first_choice) = choices.get(0) {
            if let Some(message) = first_choice.get("message") {
                if let Some(content) = message.get("content") {
                    if let Some(content_str) = content.as_str() {
                        return Ok(content_str.to_string());
                    }
                }
            }
        }
    }

    Err(anyhow!("Failed to extract text from response").into())
}