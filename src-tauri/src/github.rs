use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::errors::{MyError, Result};

pub(crate) struct GitHub {
    client: reqwest::Client,
}

impl GitHub {
    pub(crate) fn new(token: &str) -> Self {
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

    pub(crate) async fn fetch_assigned_issues(&self, organisation: &str) -> Result<Vec<Issue>> {
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

    pub(crate) async fn fetch_created_issues(&self, organisation: &str) -> Result<Vec<Issue>> {
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
pub(crate) struct Issue {
    pub(crate) title: String,
    pub(crate) number: u64,
    pub(crate) html_url: String,
    pub(crate) pull_request: Option<serde_json::Value>,
}
