use anyhow::Result;
use std::path::PathBuf;
use tokio::process::Command;

use crate::{errors::GiteaError, gitea::api::GiteaClient, github::api::GithubClient};

// Clone un dépôt Gitea et le pousse vers GitHub
pub async fn mirror_repository(
    github: &GithubClient,
    gitea: &GiteaClient,
    gitea_url: &str,
    repo_name: &str,
    is_private: bool,
    temp_dir: &PathBuf,
) -> Result<(), GiteaError> {
    if !github.repo_exists(repo_name).await? {
        github.create_repo(repo_name, is_private).await?;
    }

    // Dossier temporaire : temp_dir/repo_name
    let repo_temp_path = temp_dir.join(repo_name);
    tokio::fs::create_dir_all(&repo_temp_path).await?;

    let url = format!(
        "https://{}:{}@{}",
        gitea.gitea_name,
        gitea.token,
        gitea_url.trim_start_matches("https://"),
    );

    // Clone le dépôt Gitea en mode mirror
    let clone_status = Command::new("git")
        .arg("clone")
        .arg("--mirror")
        .arg(&url)
        .arg(&repo_temp_path)
        .status()
        .await?;

    if !clone_status.success() {
        return Err(GiteaError::GitError(format!(
            "Échec du clone depuis Gitea: {}",
            gitea_url
        )));
    }

    // Pousse vers GitHub
    let github_url = format!(
        "https://{}@github.com/{}/{}.git",
        github.token, github.username, repo_name
    );

    let push_status = Command::new("git")
        .current_dir(&repo_temp_path)
        .arg("push")
        .arg("--mirror")
        .arg(&github_url)
        .status()
        .await?;

    if !push_status.success() {
        return Err(GiteaError::GitError(format!(
            "Échec du push vers GitHub: {}",
            github_url
        )));
    }

    // Nettoie le dossier temporaire
    tokio::fs::remove_dir_all(repo_temp_path).await?;

    Ok(())
}
