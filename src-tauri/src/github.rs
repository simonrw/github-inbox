// TODO: can we unify the queries and just use the search interface?
use reqwest::{IntoUrl, Response};
use serde::{Deserialize, Deserializer, Serialize};

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

    pub(crate) async fn fetch_orgs(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct Org {
            #[serde(rename = "login")]
            name: String,
        }

        let url = "https://api.github.com/user/orgs";
        let res = self
            .client
            .get::<&str, &[&str]>(url, None)
            .await
            .map_err(|_| MyError::HttpError)?;

        let res = res.error_for_status().map_err(|e| {
            eprintln!("got bad status: {e:?}");
            MyError::BadResponse(e.status().unwrap().as_u16())
        })?;

        let orgs: Vec<Org> = res.json().await.expect("decoding result");
        Ok(orgs.into_iter().map(|o| o.name).collect())
    }

    pub(crate) async fn fetch_assigned_issues(
        &self,
        username: &str,
        organisation: &str,
    ) -> Result<Vec<Issue>> {
        self.search(format!(
            "is:open is:issue archived:false user:{organisation} assignee:{username}"
        ))
        .await
    }

    pub(crate) async fn fetch_created_issues(
        &self,
        username: &str,
        organisation: &str,
    ) -> Result<Vec<Issue>> {
        self.search(format!(
            "is:open is:issue author:{username} archived:false user:{organisation}"
        ))
        .await
    }

    pub(crate) async fn fetch_mentioned_issues(
        &self,
        username: &str,
        organisation: &str,
    ) -> Result<Vec<Issue>> {
        self.search(format!(
            "is:open is:issue archived:false user:{organisation} mentions:{username} "
        ))
        .await
    }

    pub(crate) async fn fetch_created_prs(
        &self,
        username: &str,
        organisation: &str,
    ) -> Result<Vec<Issue>> {
        self.search(format!(
            "is:open is:pr author:{username} archived:false user:{organisation}"
        ))
        .await
    }

    pub(crate) async fn fetch_assigned_prs(
        &self,
        username: &str,
        organisation: &str,
    ) -> Result<Vec<Issue>> {
        self.search(format!(
            "is:open is:pr archived:false user:{organisation} assignee:{username}"
        ))
        .await
    }

    pub(crate) async fn fetch_mentioned_prs(
        &self,
        username: &str,
        organisation: &str,
    ) -> Result<Vec<Issue>> {
        self.search(format!(
            "is:open is:pr archived:false user:{organisation} mentions:{username}"
        ))
        .await
    }

    pub(crate) async fn fetch_review_requests(
        &self,
        username: &str,
        organisation: &str,
    ) -> Result<Vec<Issue>> {
        self.search(format!(
            "is:open is:pr review-requested:{username} archived:false org:{organisation}"
        ))
        .await
    }

    async fn search(&self, query_args: impl AsRef<str>) -> Result<Vec<Issue>> {
        let url = format!("https://api.github.com/search/issues");
        let res = self
            .client
            .get(url, Some(&[("q", query_args.as_ref())]))
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
    pub(crate) draft: Option<bool>,
    #[serde(
        rename(deserialize = "repository_url"),
        deserialize_with = "deserialize_repo"
    )]
    pub(crate) repo: Repo,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Repo {
    owner: String,
    name: String,
}

fn deserialize_repo<'de, D>(deserializer: D) -> std::result::Result<Repo, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let mut parts = s.split('/').rev();
    let name = parts.next().unwrap();
    let owner = parts.next().unwrap();

    Ok(Repo {
        owner: owner.to_string(),
        name: name.to_string(),
    })
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
