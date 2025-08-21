use copypasta::ClipboardProvider;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ClipboardData {
    content: String,
}

pub async fn start_client(
    server_address: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let mut clipboard = copypasta::ClipboardContext::new()
        .map_err(|e| format!("Failed to create clipboard context: {}", e))?;

    let mut last_local_content = String::new();
    let mut last_server_content = String::new();

    println!("Starting clipboard sync with {}", server_address);

    loop {
        let current_local_content = clipboard.get_contents().unwrap_or_else(|_| String::new());

        if current_local_content != last_local_content && !current_local_content.is_empty() {
            let resp = client
                .post(format!("{}/sync", server_address))
                .json(&serde_json::json!({ "content": current_local_content }))
                .send()
                .await;

            match resp {
                Ok(response) if response.status().is_success() => {
                    println!(
                        "Sent to server: \"{}\"",
                        if current_local_content.len() > 50 {
                            format!("{}...", &current_local_content[..50])
                        } else {
                            current_local_content.clone()
                        }
                    );
                    last_local_content = current_local_content.clone();
                    last_server_content = current_local_content.clone(); // Avoid immediate pull-back
                }
                Ok(_) => eprintln!("❌ Error sending clipboard to server"),
                Err(e) => eprintln!("❌ Network error sending to server: {}", e),
            }
        }

        let server_resp = client.get(format!("{}/get", server_address)).send().await;

        match server_resp {
            Ok(response) if response.status().is_success() => {
                if let Ok(server_data) = response.json::<ClipboardData>().await {
                    // Update local clipboard if server has different content
                    if server_data.content != last_server_content
                        && server_data.content != current_local_content
                        && !server_data.content.is_empty()
                    {
                        match clipboard.set_contents(server_data.content.clone()) {
                            Ok(_) => {
                                println!(
                                    "Received from server: \"{}\"",
                                    if server_data.content.len() > 50 {
                                        format!("{}...", &server_data.content[..50])
                                    } else {
                                        server_data.content.clone()
                                    }
                                );
                                last_server_content = server_data.content.clone();
                                last_local_content = server_data.content;
                            }
                            Err(e) => eprintln!("❌ Failed to set clipboard: {}", e),
                        }
                    }
                }
            }
            Ok(response) if response.status().as_u16() == 404 => {}
            Ok(_) => eprintln!("❌ Unexpected response from server"),
            Err(e) => eprintln!("❌ Network error getting from server: {}", e),
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
