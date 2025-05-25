# Gitea to GitHub Migrator

Un outil en ligne de commande robuste pour migrer des dépôts de Gitea vers GitHub, écrit en Rust.

## 📋 Description

Cet outil permet de migrer facilement et efficacement vos dépôts depuis une instance Gitea vers GitHub. Il gère la migration complète, y compris les métadonnées du projet, les issues, et l'historique Git.

## ✨ Fonctionnalités

- Migration des dépôts Gitea vers GitHub
- Support de l'authentification sécurisée
- Barre de progression pour suivre l'avancement
- Gestion des erreurs robuste
- Interface en ligne de commande intuitive

## 🔧 Prérequis

- Rust (édition 2024)
- Git installé sur votre système
- Accès à votre instance Gitea (URL et token)
- Compte GitHub et token d'accès personnel

## 📦 Installation

1. Clonez le dépôt :
```bash
git clone https://github.com/waldo-st/gitea-to-github-migrator.git
cd gitea-to-github-migrator
```
2. Compilez et Installez globalement le projet:

- #### Linux et macOS
```bash
# Compilation
cargo build --release

# Assurez-vous que l'exécutable de base a les bonnes permissions
sudo chmod +x target/release/gitea-to-github-migrator

# Installation globale (nécessite les droits sudo)
sudo ln -s "$(pwd)/target/release/gitea-to-github-migrator" /usr/local/bin/g2gh
sudo chmod 755 /usr/local/bin/g2gh
```

- #### Windows (cmd.exe)
```batch
:: Compilation
cargo build --release

:: Créer le dossier bin si nécessaire
if not exist "%USERPROFILE%\bin" mkdir "%USERPROFILE%\bin"

:: Copier l'exécutable
copy "target\release\gitea-to-github-migrator.exe" "%USERPROFILE%\bin\g2gh.exe"

:: Ajouter au PATH (nécessite des droits administrateur)
setx PATH "%PATH%;%USERPROFILE%\bin"

echo Installation terminee ! Redemarrez votre terminal pour utiliser la commande g2gh
```

3. verifiez l'installation
```bash
# commande help, pour avoir de l'aide
g2gh --help
# Ou
g2gh -h
```
Maintenant vous pouvez utiliser la commande raccourcie `g2gh` au lieu du nom complet.

## 🚀 Utilisation

### Configuration

Avant d'utiliser l'outil, vous aurez besoin de :

1. URL de votre instance Gitea (Ex: learn.zone01dakar.sn)
2. Token d'accès Gitea
3. Token d'accès GitHub
4. Nom d'utilisateur GitHub

### Commandes

L'outil s'utilise via la ligne de commande avec les syntaxes suivantes :
```bash
# Pour creer un token d'acces Gitea et le stocker en toute securité dans un dossier .gitea
g2gh -s <URL de votre instance Gitea>
#ou
g2gh --show <URL de votre instance Gitea>

# Pour migrer un dépot Gitea vers GitHub
g2gh -r <Nom de votre Dépot> -m <URL de votre instance Gitea> -n <Votre nom d\'utilisateur Gitea>
# Ou
g2gh --repo <Nom de votre Dépot> --migrate <URL de votre instance Gitea> --name <Votre nom d\'utilisateur Gitea>

# Pour migrer tous vos dépots Gitea vers GitHub
g2gh -a -m <URL de votre instance Gitea> -n <Votre nom d\'utilisateur Gitea>
# Ou
g2gh --all --migrate <URL de votre instance Gitea> --name <Votre nom d\'utilisateur Gitea>
```

🔒 Sécurité
- Les tokens sont gérés de manière sécurisée
- Les mots de passe sont masqués lors de la saisie
- Les informations sensibles ne sont jamais enregistrées en clair

## 🛠 Structure du Projet
```bash
gitea-to-github-migrator/
├── src/
│   ├── cli.rs          # Gestion des arguments en ligne de commande
│   ├── errors.rs       # Gestion des erreurs personnalisées
│   ├── git/            # Opérations Git
│   ├── gitea/          # API et modèles Gitea
│   ├── github/         # API et modèles GitHub
│   ├── progress/       # Affichage de la progression
│   ├── tests/          # Tests unitaires
│   ├── utils.rs        # Utilitaires
│   ├── lib.rs          # Bibliothèque principale
│   └── main.rs         # Point d'entrée
```
📚 Dépendances Principales

- `reqwest` : Client HTTP asynchrone
- `tokio` : Runtime asynchrone
- `serde` : Sérialisation/désérialisation
- `clap` : Parsing des arguments CLI
- `indicatif` : Barres de progression
- `dialoguer` : Interface interactive
- `colored` : Mise en forme des sorties console

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :
1. Fork le projet
2. Créer une branche pour votre fonctionnalité
3. Commiter vos changements
4. Pousser vers la branche
5. Ouvrir une Pull Request

## 📝 Licence

Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus de détails.

⚠️ Notes

- Vérifiez vos permissions sur les deux plateformes
- Testez d'abord sur un petit dépôt avant de migrer des projets importants
- Si vous voulez désinstaller l'outil plus tard
```bash
sudo rm /usr/local/bin/g2gh
```
