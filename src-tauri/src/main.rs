// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::Engine;
use serde_json::json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn generate_image(api_token: String, prompt: String) -> String {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/images/generations")
        .bearer_auth(api_token)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "prompt": prompt,
                "n": 1,
                "size": "1024x1024"
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap();

    // Really bad stripping
    let image_url_json =
        serde_json::from_slice::<serde_json::Value>(&response.bytes().await.unwrap()).unwrap()
            ["data"][0]["url"]
            .to_string();
    let image_url = image_url_json.trim_matches('"');

    println!("{}", image_url);

    let image = reqwest::get(image_url)
        .await
        .unwrap()
        .bytes()
        .await
        .map(|bytes| bytes.to_vec())
        .unwrap();

    let engine = base64::engine::general_purpose::STANDARD_NO_PAD;

    engine.encode(image)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
