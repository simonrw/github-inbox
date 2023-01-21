use reqwest::{IntoUrl, Response};
use serde::{Deserialize, Serialize};

use crate::errors::{MyError, Result};

#[async_trait::async_trait]
pub(crate) trait HttpClient {
    async fn get<U, T>(&self, url: U, params: Option<&T>) -> Result<Response>
    where
        U: IntoUrl + Send,
        T: Serialize + Sync + ?Sized;
}

#[async_trait::async_trait]
impl HttpClient for reqwest::Client {
    async fn get<U, T>(&self, url: U, params: Option<&T>) -> Result<Response>
    where
        U: IntoUrl + Send,
        T: Serialize + Sync + ?Sized,
    {
        let mut req = self.get(url);
        if let Some(p) = params {
            req = req.query(p);
        }
        req.send().await.map_err(|_| MyError::HttpError)
    }
}

pub(crate) struct GitHub<H> {
    client: H,
}

impl<H> GitHub<H>
where
    H: HttpClient,
{
    pub(crate) fn new(http_client: H) -> Self {
        Self {
            client: http_client,
        }
    }

    pub(crate) async fn fetch_assigned_issues(&self, organisation: &str) -> Result<Vec<Issue>> {
        let url = format!("https://api.github.com/orgs/{organisation}/issues");
        let res = self
            .client
            .get::<String, &[&str]>(url, None)
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
            .get(url, Some(&[("filter", "created")]))
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

    pub(crate) async fn fetch_mentioned_issues(&self, organisation: &str) -> Result<Vec<Issue>> {
        let url = format!("https://api.github.com/orgs/{organisation}/issues");
        let res = self
            .client
            .get(url, Some(&[("filter", "mentioned")]))
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

    pub(crate) async fn fetch_review_requests(&self, organisation: &str) -> Result<Vec<Issue>> {
        let url = format!("https://api.github.com/search/issues");
        let res = self
            .client
            .get(
                url,
                Some(&[("q", "is:open is:pr review-requested:simonrw archived:false")]),
            )
            .await
            .map_err(|_| MyError::HttpError)?;

        let res = res.error_for_status().map_err(|e| {
            eprintln!("got bad status: {e:?}");
            MyError::BadResponse(e.status().unwrap().as_u16())
        })?;

        let issues: Vec<Issue> = {
            #[derive(Deserialize)]
            struct TempResponse {
                items: Vec<Issue>,
            }

            let response: TempResponse = res.json().await.expect("decoding result");
            response.items
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     struct MockClient {}

//     #[async_trait::async_trait]
//     impl HttpClient for MockClient {
//         async fn get<U, T>(&self, url: U, params: Option<&T>) -> Result<Response>
//         where
//             U: IntoUrl + Send,
//             T: Serialize + Sync + ?Sized,
//         {
//             todo!()
//         }
//     }

//     #[tokio::test]
//     async fn fetching_created_issues() {}
// }
