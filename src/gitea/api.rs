use std::{fs, sync::Arc};

use anyhow::Result;
use reqwest::Client;
use tokio::sync::Semaphore;

use crate::{
    errors::GiteaError,
    utils::{get_token_file_path, read_user_input},
};

use super::models::{GiteaRepo, TokenResponse};

pub struct GiteaClient {
    pub client: Client,
    pub gitea_url: String,
    pub gitea_name: String,
    pub token: String,
    pub semaphore: Arc<Semaphore>,
}

impl GiteaClient {
    pub fn new(gitea_name: String, url_gitea: String, max_parellel: usize) -> Self {
        let gitea_token = read_user_input(&format!("Enter Gitea token"), true).unwrap();

        Self {
            client: Client::new(),
            gitea_url: url_gitea,
            gitea_name,
            token: gitea_token,
            semaphore: Arc::new(Semaphore::new(max_parellel)),
        }
    }

    pub async fn fetch_all_repos(&self) -> Result<Vec<GiteaRepo>, GiteaError> {
        let _permit = self.semaphore.acquire().await?;
        let url = format!(
            "https://{}/git/api/v1/users/{}/repos",
            self.gitea_url, self.gitea_name
        );

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()?;

        let repos: Vec<GiteaRepo> = response.json().await?;

        Ok(repos)
    }

    pub async fn fetch_repo(&self, repo_name: &str) -> Result<GiteaRepo, GiteaError> {
        let _permit = self.semaphore.acquire().await?;

        let url = format!(
            "https://{}/git/api/v1/repos/{}/{}",
            self.gitea_url, self.gitea_name, repo_name
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()?;

        let repo: GiteaRepo = response.json().await?;
        Ok(repo)
    }
}

async fn request_new_token(
    base_url: &str,
    username: &str,
    password: &str,
) -> Result<String, GiteaError> {
    let client = Client::new();
    let url = format!("https://{}/git/api/v1/users/{}/tokens", base_url, username);

    let name_token = format!("token_{}", chrono::Utc::now().timestamp());
    
    let payload = serde_json::json!({
        "name": name_token,
        "scopes": [
            "read:activitypub",
            "read:issue",
            "write:misc",
            "read:notification",
            "read:organization",
            "read:package",
            "read:repository",
            "read:user"
        ]
    });

    let response = client
        .post(&url)
        .basic_auth(username, Some(password))
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    let token_data: TokenResponse = response.json().await?;
    Ok(token_data.sha1)
}

pub async fn get_or_create_token_gitea(url_gitea: &str) -> Result<String, GiteaError> {
    let token_file = get_token_file_path();

    let token = fs::read_to_string(&token_file)
        .map_err(|e| GiteaError::IoError(e))?
        .trim()
        .to_string();

    if token.trim().is_empty() {
        let gitea_name = read_user_input("Enter your Gitea username: ", false)?;
        let gitea_password =
            read_user_input(&format!("Enter Gitea password for {}", gitea_name), true)?;

        let new_token = request_new_token(url_gitea, &gitea_name, &gitea_password).await?;

        fs::write(&token_file, &new_token).map_err(|e| GiteaError::IoError(e))?;

        return Ok(new_token);
    } else {
        return Ok(token);
    };
}
