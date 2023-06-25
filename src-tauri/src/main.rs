// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use action_handler::{ActionDispatcher};
use repository::surreal_repository::SurrealRepository;
use surrealdb::{Surreal, engine::local::File};
use tauri::{State, Manager};

mod value_objects;
mod repository;
mod model;
mod action_handler;
mod actions;

use std::{string::String, collections::HashMap, sync::Arc};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::string::ToString;

use model::combatten_service::{CombattenService};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn ipc_message(message: IpcMessage, context: State<'_, ApplicationContext>) -> Result<IpcMessage, ()> {
    let dispatcher = context.action_dispatchers.get(&message.domain).unwrap();
    let response = dispatcher.dispatch_action(message.domain.to_string(),message.action).await;
    Ok(IpcMessage {
        domain: message.domain,
        action: response
    })
}
fn main() {
    let context = ApplicationContext::new();

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(context)
        .invoke_handler(tauri::generate_handler![ipc_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Deserialize, Serialize)]
struct IpcMessage {
    domain: String,
    action: Value
} 

struct ApplicationContext {
    action_dispatchers: HashMap<String, Arc<dyn ActionDispatcher + Sync + Send>>
}

impl ApplicationContext {
    async fn new() -> Self { 
        let surreal_db = Surreal::new::<File>("testdata/surreal/initiative.db").await.unwrap();
        surreal_db.use_ns("runners").use_db("manager").await.unwrap();
        let repository = Box::new(SurrealRepository::new(Box::new(surreal_db), "classifiers"));
        let service = Arc::new(CombattenService::new(repository));
        let mut action_dispatchers: HashMap<String, Arc<dyn ActionDispatcher + Sync + Send>> = HashMap::new();
        action_dispatchers.insert(actions::combatten_action::COMBATTEN_DOMAIN.to_string(), service.clone());
        action_dispatchers.insert(actions::application_action::APPLICATION_DOMAIN.to_string(), service.clone());
        Self { action_dispatchers }
    }
}
