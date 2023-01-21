#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod errors;
mod github;

use crate::errors::Result;
use crate::github::{GitHub, Issue};

#[tauri::command]
async fn fetch_assigned_issues(
    gh: tauri::State<'_, GitHub>,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_assigned_issues(&organisation).await
}

#[tauri::command]
async fn fetch_created_issues(
    gh: tauri::State<'_, GitHub>,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_created_issues(&organisation).await
}

fn main() {
    tracing_subscriber::fmt::init();
    let token = std::env::var("GITHUB_TOKEN").unwrap();
    let gh = GitHub::new(&token);
    tauri::Builder::default()
        .manage(gh)
        .invoke_handler(tauri::generate_handler![
            fetch_assigned_issues,
            fetch_created_issues
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
