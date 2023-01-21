#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod errors;
mod github;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::errors::Result;
use crate::github::{GitHub, Issue};

#[tauri::command]
async fn fetch_orgs(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
) -> Result<Vec<String>> {
    gh.fetch_orgs().await
}

#[tauri::command]
async fn fetch_assigned_issues(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_assigned_issues(&organisation).await
}

#[tauri::command]
async fn fetch_created_issues(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_created_issues(&organisation).await
}

#[tauri::command]
async fn fetch_mentioned_issues(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_mentioned_issues(&organisation).await
}

#[tauri::command]
async fn fetch_created_prs(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_created_prs(&organisation).await
}

#[tauri::command]
async fn fetch_assigned_prs(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_assigned_prs(&organisation).await
}

#[tauri::command]
async fn fetch_mentioned_prs(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_mentioned_prs(&organisation).await
}

#[tauri::command]
async fn fetch_review_requests(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_review_requests(&organisation).await
}

fn main() {
    tracing_subscriber::fmt::init();
    let token = std::env::var("GITHUB_TOKEN").unwrap();
    let mut headers = HeaderMap::new();
    let mut auth_value = HeaderValue::from_str(&format!("Bearer {token}")).unwrap();
    auth_value.set_sensitive(true);
    headers.insert("Authorization", auth_value);
    headers.insert("User-Agent", HeaderValue::from_static("github-inbox/0.1.0"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let gh = GitHub::new(client);
    tauri::Builder::default()
        .manage(gh)
        .invoke_handler(tauri::generate_handler![
            fetch_orgs,
            fetch_created_issues,
            fetch_assigned_issues,
            fetch_mentioned_issues,
            fetch_created_prs,
            fetch_assigned_prs,
            fetch_mentioned_prs,
            fetch_review_requests,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
