use std::{fs, io::Write, path::PathBuf};

use crate::errors::GiteaError;
use anyhow::Result;
use colored::*;
use dialoguer::Password;

pub fn get_token_file_path() -> PathBuf {
    let mut dir = dirs::home_dir().expect("Impossible de localiser le répertoire home");
    dir.push(".gitea");
    fs::create_dir_all(&dir).expect("Erreur lors de la création du dossier .gitea");
    dir.push("token.txt");
    dir
}

pub fn read_user_input(prompt: &str, is_password: bool) -> Result<String, GiteaError> {
    if is_password {
        Ok(Password::new()
            .with_prompt(prompt)
            .with_confirmation("Confirm password", "Passwords do not match")
            .interact()
            .map_err(|e| GiteaError::DialoguerError(e.to_string()))?)
    } else {
        // Pour les saisies normales
        print!("{}", prompt);
        std::io::stdout().flush().map_err(GiteaError::IoError)?;
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(GiteaError::IoError)?;
        Ok(input.trim().to_string())
    }
}

pub fn print_logo() {
    println!(
        "          {}          ",
        "   ____  ____    ____  _   _ ".bright_blue().bold()
    );
    println!(
        "          {}          ",
        "  / ___||__  \\  / ___|| | | |".bright_blue().bold()
    );
    println!(
        "          {}          ",
        " | |  _   / /  | |  _ | |_| |".cyan().bold()
    );
    println!(
        "          {}          ",
        " | |_| | / /_  | |_| ||  _  |".cyan().bold()
    );
    println!(
        "          {}          ",
        "  \\____||____|  \\____||_| |_| ".bright_blue().bold()
    );

    println!(
        "\n{} {}\n",
        "🚀".green(),
        "Gitea → GitHub migration made simple with g2gh"
            .yellow()
            .bold()
    );
}

pub fn print_token(token: &str) {
    println!(
        "{}",
        "\n=================================================".bright_blue()
    );
    println!(
        "{}",
        "              GITEA ACCESS TOKEN                   ".cyan()
    );
    println!(
        "{}",
        "=================================================".bright_blue()
    );
    println!("  {}  ", token.cyan());
    print!(
        "{}",
        "=================================================\n".bright_blue()
    );
}
