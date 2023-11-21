// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest;
use serde::{Deserialize};


// 2. Add the Pokemon structure


#[tauri::command]
async fn get_pokemon(name: &str) -> Result<String, String> {
    // The URL of the Pokemon API with the provided name
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);

    // Make the GET request
    match reqwest::get(&url).await {
        Ok(response) => {

            if response.status().is_success() {

                // 1. Get the response body as text

                // 3. Parse the response body to JSON and values to struct

                Ok(format!("Todo"))
               
            } else {
                Err(format!("Error: HTTP status code {}", response.status()))
            }
        }
        Err(err) => Err(format!("Error making HTTP request: {}", err)),
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_pokemon])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
