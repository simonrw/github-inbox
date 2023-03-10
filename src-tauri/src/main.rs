#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod errors;
mod github;

use github::Notification;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::errors::Result;
use crate::github::{GitHub, Issue};

#[tauri::command]
async fn fetch_orgs(gh: tauri::State<'_, GitHub<reqwest::Client>>) -> Result<Vec<String>> {
    gh.fetch_orgs().await
}

#[tauri::command]
async fn fetch_assigned_issues(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    username: String,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_assigned_issues(&username, &organisation).await
}

#[tauri::command]
async fn fetch_created_issues(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    username: String,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_created_issues(&username, &organisation).await
}

#[tauri::command]
async fn fetch_mentioned_issues(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    username: String,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_mentioned_issues(&username, &organisation).await
}

#[tauri::command]
async fn fetch_created_prs(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    username: String,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_created_prs(&username, &organisation).await
}

#[tauri::command]
async fn fetch_assigned_prs(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    username: String,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_assigned_prs(&username, &organisation).await
}

#[tauri::command]
async fn fetch_mentioned_prs(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    username: String,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_mentioned_prs(&username, &organisation).await
}

#[tauri::command]
async fn fetch_review_requests(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    username: String,
    organisation: String,
) -> Result<Vec<Issue>> {
    gh.fetch_review_requests(&username, &organisation).await
}

#[tauri::command]
async fn fetch_notifications(
    gh: tauri::State<'_, GitHub<reqwest::Client>>,
    username: String,
) -> Result<Vec<Notification>> {
    gh.fetch_notifications(&username).await
}

fn main() {
    tracing_subscriber::fmt::init();
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("github-inbox/0.1.0"));

    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        let mut auth_value = HeaderValue::from_str(&format!("Bearer {token}")).unwrap();
        auth_value.set_sensitive(true);
        headers.insert("Authorization", auth_value);
    } else {
        tracing::warn!("no token found - viewing only public issues and PRs");
    }

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
            fetch_notifications,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
