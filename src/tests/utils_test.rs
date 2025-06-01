use std::fs;
use std::path::PathBuf;

use crate::utils::get_token_file_path;

#[test]
fn test_get_token_file_path() {
    // Appeler la fonction
    let token_path: PathBuf = get_token_file_path();
    println!("Chemin du fichier token : {:?}", token_path);
    // Vérifier que le chemin est valide
    assert!(token_path.is_absolute(), "Le chemin doit être absolu.");
    assert!(
        token_path.ends_with("token.txt"),
        "Le fichier doit être 'token.txt'."
    );

    // Vérifier que le répertoire .gitea a été créé
    let dir = token_path
        .parent()
        .expect("Le chemin doit avoir un parent.");
    assert!(dir.exists(), "Le répertoire .gitea doit exister.");
    assert!(dir.is_dir(), "Le chemin doit être un répertoire.");

    // Vérifier que le fichier token.txt existe
    assert!(token_path.exists(), "Le fichier token.txt doit exister.");
    assert!(token_path.is_file(), "Le chemin doit être un fichier.");

    // Nettoyage (supprimer le fichier et le répertoire pour éviter les conflits)
    fs::remove_file(&token_path).expect("Impossible de supprimer le fichier token.txt.");
    fs::remove_dir_all(dir).expect("Impossible de supprimer le répertoire .gitea.");
}
