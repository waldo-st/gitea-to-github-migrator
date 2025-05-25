use gitea_to_github_migrator::cli::{parse_args, run};
use gitea_to_github_migrator::utils::print_logo;

#[tokio::main]
async fn main() {
    print_logo();

    let matches = parse_args();

    if let Err(e) = run(matches).await {
        eprintln!("Erreur : {}", e);
        std::process::exit(1);
    }
}
