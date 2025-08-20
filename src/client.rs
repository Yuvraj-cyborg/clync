use copypasta::ClipboardProvider;

pub async fn start_client(
    server_address: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let mut clipboard = copypasta::ClipboardContext::new()
        .map_err(|e| format!("Failed to create clipboard context: {}", e))?;

    loop {
        let content = clipboard
            .get_contents()
            .map_err(|e| format!("Failed to get clipboard contents: {}", e))?;

        let resp = client
            .post(format!("{}/sync", server_address))
            .json(&serde_json::json!({ "content": content }))
            .send()
            .await?;

        if resp.status().is_success() {
            println!("Clipboard synced successfully!");
        } else {
            eprintln!("Error syncing clipboard!");
        }

        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}
