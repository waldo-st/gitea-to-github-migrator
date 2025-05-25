use anyhow::Result;
use reqwest::Client;

use crate::{errors::GiteaError, utils::read_user_input};

use super::models::CreateRepoRequest;

pub struct GithubClient {
    pub client: Client,
    pub username: String,
    pub token: String,
}

impl GithubClient {
    pub fn new() -> Self {
        let github_user = read_user_input(&format!("Enter Github username : "), false).unwrap();
        let github_token = read_user_input(&format!("Enter Github token"), true).unwrap();
        Self {
            client: Client::new(),
            username: github_user,
            token: github_token,
        }
    }

    pub async fn create_repo(&self, name_repo: &str, is_private: bool) -> Result<(), GiteaError> {
        let request = CreateRepoRequest {
            name: name_repo.to_string(),
            private: is_private,
        };

        let response = self
            .client
            .post("https://api.github.com/user/repos")
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "g2gh-cli/1.0")
            .header("Accept", "application/vnd.github+json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_msg = response.text().await.unwrap_or_default();
            return Err(GiteaError::ApiError(format!(
                "Failed to create repo [{}]: {}",
                status, error_msg
            )));
        }

        Ok(())
    }

    pub async fn repo_exists(&self, repo_name: &str) -> Result<bool, GiteaError> {
        let url = format!(
            "https://api.github.com/repos/{}/{}",
            self.username, repo_name
        );
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "g2gh-cli/1.0")
            .send()
            .await?;

        Ok(response.status().is_success())
    }
}
