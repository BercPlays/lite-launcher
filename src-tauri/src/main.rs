// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use api_handler::api_handlers::ApiHandler;
use discord_rpc_handler::rpc_handler::{DefinedActivities, RPCHandler};

mod api_handler;
mod backend_types;
mod discord_rpc_handler;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let mut rpc = RPCHandler::new("1228620742095016018");
    let _ = match rpc.connect() {
        Ok(()) => rpc.set_activity(DefinedActivities::in_launcher()),
        Err(err) => Err(err),
    };

    tauri::Builder::default()
        .manage(ApiHandler {
            client: reqwest::Client::new(),
        })
        .invoke_handler(tauri::generate_handler![get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get(state: tauri::State<'_, ApiHandler>, invoke_message: String) -> Result<String, ()> {
    let response = state.get(&invoke_message).await;

    let text = match response {
        Ok(text) => text,
        Err(error) => error,
    };
    Ok(text)
}
