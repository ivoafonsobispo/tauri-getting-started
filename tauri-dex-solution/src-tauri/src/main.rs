// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Pokemon {
    id: u32,
    name: String,
    weight: u32,
    height: u32,
    // Add other fields you might need from the response
    // For example, you might want to extract the name: String,
}

#[tauri::command]
async fn get_pokemon(name: &str) -> Result<String, String> {
    // The URL of the Pokemon API with the provided name
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);

    // Make the GET request
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                // If the request was successful, get the response body as text
                let body = response.text().await.unwrap();

                // Parse the JSON body
                match serde_json::from_str::<Pokemon>(&body) {
                    Ok(pokemon) => {
                        // Create a Pokemon Object
                        let pokemon_object = Pokemon {
                            id: pokemon.id,
                            name: pokemon.name,
                            weight: pokemon.weight,
                            height: pokemon.height,
                        };

                        Ok(format!("{:?}", pokemon_object))
                    }
                    Err(err) => Err(format!("Error parsing JSON: {}", err)),
                }
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
