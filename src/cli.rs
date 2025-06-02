use std::env;

use crate::{
    errors::GiteaError,
    git::mirror::{MirrorResult, mirror_repository},
    gitea::api::{GiteaClient, get_or_create_token_gitea},
    github::api::GithubClient,
    progress::multi_progress::{add_progress, setup_progress},
    utils::print_token,
};
use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

//  Definit et analyse les arguments de ligne de commande
pub fn parse_args() -> ArgMatches {
    Command::new("Gitea to GitHub Migrator")
        .version("1.0.0")
        .author("waldo-st")
        // .about("Migrated repositories from Gitea to GitHub")
        .arg(
            Arg::new("url-gitea")
                .short('m')
                .long("migrate")
                .value_name("URL GITEA")
                .help("Migration of Gitea repositories to GitHub")
                .action(clap::ArgAction::Set)
                .required_unless_present("show-token"),
        )
        .arg(
            Arg::new("gitea-name")
                .short('n')
                .long("name")
                .value_name("USERNAME")
                .help("Your Gitea username")
                .action(clap::ArgAction::Set)
                .required_unless_present("show-token"),
        )
        .arg(
            Arg::new("all-repos")
                .short('a')
                .long("all")
                .help("Migrate all repositories")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("single-repo"),
        )
        .arg(
            Arg::new("single-repo")
                .short('r')
                .long("repo")
                .value_name("REPO_NAME")
                .help("Migrate a single repository")
                .action(clap::ArgAction::Set)
                .conflicts_with("all-repos"),
        )
        .arg(
            Arg::new("show-token")
                .short('s')
                .long("show")
                .value_name("URL GITEA")
                .help("Show the Gitea token")
                .action(clap::ArgAction::Set)
                .conflicts_with_all(["gitea-name", "url-gitea"]),
        )
        .get_matches()
}

//Exécute la logique principale en fonction des arguments.
pub async fn run(matches: ArgMatches) -> Result<(), GiteaError> {
    let m = setup_progress();

    if let Some(url_gitea) = matches.get_one::<String>("show-token") {
        match get_or_create_token_gitea(url_gitea).await {
            Ok(token) => {
                print_token(&token);
                return Ok(());
            }
            Err(e) => {
                eprintln!("Erreur {}", e);
                return Err(e);
            }
        }
    }
    let url_gitea = matches
        .get_one::<String>("url-gitea")
        .ok_or(GiteaError::MissingArguments)?;

    let gitea_name = matches
        .get_one::<String>("gitea-name")
        .ok_or(GiteaError::MissingArguments)?;

    let gitea_client = GiteaClient::new(gitea_name.into(), url_gitea.into(), 4);
    let github_client = GithubClient::new();
    let temp_dir = env::temp_dir().join("gitea_migrator");

    if matches.get_flag("all-repos") {
        let repos = gitea_client.fetch_all_repos().await?;
        let pb = add_progress(
            &m,
            repos.len() as u64,
            &format!("Migrations de `{}` repositories...", repos.len()),
        );

        let mut migrated_count = 0;
        let mut skipped_count = 0;

        for repo in repos {
            let result = mirror_repository(
                &github_client,
                &gitea_client,
                &repo.clone_url,
                &repo.name,
                repo.private,
                &temp_dir,
            )
            .await?;

            match result {
                MirrorResult::Success => {
                    migrated_count += 1;
                }
                MirrorResult::AlreadyExists => {
                    skipped_count += 1;
                }
            }
            pb.inc(1);
        }

        pb.finish_with_message(format!(
            "✅ Migration terminée: {} migrés, {} déjà existants",
            migrated_count, skipped_count
        ));
        tokio::fs::remove_dir_all(&temp_dir).await?;
    } else if let Some(repo_name) = matches.get_one::<String>("single-repo") {
        let pb = add_progress(&m, 1, &format!("Migration de `{}`", repo_name));

        let repo = gitea_client.fetch_repo(repo_name).await?;

        let result = mirror_repository(
            &github_client,
            &gitea_client,
            &repo.clone_url,
            &repo.name,
            repo.private,
            &temp_dir,
        )
        .await?;

        pb.inc(1);

        match result {
            MirrorResult::Success => {
                pb.finish_with_message(format!("✅ Succès de la migration: {}", repo.name));
            }
            MirrorResult::AlreadyExists => {
                pb.finish_with_message(format!(
                    "ℹ️ Le dépôt '{}' existe déjà sur GitHub",
                    repo.name
                ));
            }
        }

        tokio::fs::remove_dir_all(&temp_dir).await?;
    } else {
        return Err(GiteaError::MissingArguments);
    }

    Ok(())
}
