#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
enum MyError {
    #[error("sending HTTP request")]
    HttpError,

    #[error("bad response: {0}")]
    BadResponse(u16),
}

type Result<T> = std::result::Result<T, MyError>;

struct GitHub {
    client: reqwest::Client,
}

impl GitHub {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        let mut auth_value = HeaderValue::from_str(&format!("Bearer {token}")).unwrap();
        auth_value.set_sensitive(true);
        headers.insert("Authorization", auth_value);
        headers.insert("User-Agent", HeaderValue::from_static("github-inbox/0.1.0"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn fetch_assigned_issues(&self, organisation: &str) -> Result<Vec<Issue>> {
        let url = format!("https://api.github.com/orgs/{organisation}/issues");
        let res = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| MyError::HttpError)?;

        let res = res.error_for_status().map_err(|e| {
            eprintln!("got bad status: {e:?}");
            MyError::BadResponse(e.status().unwrap().as_u16())
        })?;

        let issues: Vec<Issue> = {
            let all_entries: Vec<Issue> = res.json().await.expect("decoding result");
            // entries contains some PRs, so exclude those
            all_entries
                .into_iter()
                .filter(|e| e.pull_request.is_none())
                .collect()
        };
        Ok(issues)
    }

    pub async fn fetch_created_issues(&self, organisation: &str) -> Result<Vec<Issue>> {
        let url = format!("https://api.github.com/orgs/{organisation}/issues");
        let res = self
            .client
            .get(url)
            .query(&[("filter", "created")])
            .send()
            .await
            .map_err(|_| MyError::HttpError)?;

        let res = res.error_for_status().map_err(|e| {
            eprintln!("got bad status: {e:?}");
            MyError::BadResponse(e.status().unwrap().as_u16())
        })?;

        let issues: Vec<Issue> = {
            let all_entries: Vec<Issue> = res.json().await.expect("decoding result");
            // entries contains some PRs, so exclude those
            all_entries
                .into_iter()
                .filter(|e| e.pull_request.is_none())
                .collect()
        };
        Ok(issues)
    }
}

#[derive(Deserialize, Serialize)]
struct Issue {
    title: String,
    number: u64,
    html_url: String,
    pull_request: Option<serde_json::Value>,
}

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
