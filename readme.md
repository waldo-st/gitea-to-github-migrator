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
- Script de build et d'installation automatisé

## 🔧 Prérequis

- Rust (édition 2024)
- Git installé sur votre système
- Accès à votre instance Gitea (URL et token)
- Compte GitHub et token d'accès personnel

## 📦 Installation

### Méthode recommandée : Script automatisé

1. Clonez le dépôt :
```bash
git clone https://github.com/waldo-st/gitea-to-github-migrator.git
cd gitea-to-github-migrator
```

2. Rendez le script exécutable :
```bash
chmod +x build_and_run.sh
```

3. Compilez et installez globalement en une seule commande :
```bash
# Build en mode release et installation globale
./build_and_run.sh --release --install
```

4. Vérifiez l'installation :
```bash
g2gh --help
```

### Options du script de build

Le script `build_and_run.sh` offre plusieurs options pratiques :

```bash
# Options disponibles
./build_and_run.sh [OPTIONS] [-- APP_ARGS]

Options du script de build:
  -c, --clean       Nettoyer avant de builder
  -r, --release     Builder en mode release
  -t, --test        Exécuter les tests
  -l, --lint        Linter le code avec Clippy
  -f, --fmt         Formater le code
  -n, --no-run      Ne pas exécuter après le build
  -i, --install     Installer l'outil globalement comme 'g2gh'
  -u, --uninstall   Désinstaller l'outil global
  -h, --help        Afficher l'aide du script

# Exemples d'utilisation
./build_and_run.sh                            # Build debug et exécute
./build_and_run.sh --release                  # Build release et exécute
./build_and_run.sh --clean --test --lint      # Nettoie, teste, linte, build et exécute
./build_and_run.sh --release --install        # Build release et installe globalement
./build_and_run.sh --uninstall                # Désinstalle l'outil global
./build_and_run.sh --no-run                   # Build seulement
./build_and_run.sh -- --help                  # Build et exécute avec --help
```

### Installation manuelle (alternative)

Si vous préférez l'installation manuelle :

#### Linux et macOS
```bash
# Compilation
cargo build --release

# Installation globale (nécessite les droits sudo)
sudo ln -s "$(pwd)/target/release/gitea-to-github-migrator" /usr/local/bin/g2gh
sudo chmod 755 /usr/local/bin/g2gh
```

#### Windows (cmd.exe)
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

## 🔑 Configuration des tokens

### Permissions requises pour les tokens

#### Token Gitea (source)
- ✅ **Repository** : `read` (pour lire les dépôts et métadonnées)
- ✅ **Issue** : `read` (pour migrer les issues)
- ✅ **Pull Request** : `read` (pour migrer les PR)
- ✅ **Release** : `read` (pour migrer les releases)
- ✅ **User** : `read` (pour accéder aux infos utilisateur)
- ✅ **Organization** : `read` (si migration d'organisation)

**Comment créer un token Gitea :**
1. Allez dans `Settings → Applications → Access Tokens`
2. Créez un nouveau token avec les permissions `read` pour tous les scopes
3. Copiez le token généré

#### Token GitHub (destination)
- ✅ **repo** (accès complet aux dépôts)
- ✅ **workflow** (si migration des GitHub Actions)
- ✅ **user:email** (accès aux adresses email)
- ✅ **admin:org** (si migration vers une organisation)

**Comment créer un token GitHub :**
1. Allez dans `Settings → Developer settings → Personal access tokens → Tokens (classic)`
2. Créez un token avec les scopes : `repo`, `workflow`, `read:org`, `user:email`
3. ⚠️ Utilisez des **tokens classiques** plutôt que des fine-grained tokens
4. Copiez le token généré

## 🚀 Utilisation

### Configuration

Avant d'utiliser l'outil, vous aurez besoin de :

1. URL de votre instance Gitea (Ex: learn.zone01dakar.sn)
2. Token d'accès Gitea (avec les bonnes permissions)
3. Token d'accès GitHub (classic token)
4. Nom d'utilisateur GitHub

### Commandes

L'outil s'utilise via la ligne de commande avec les syntaxes suivantes :

```bash
# Pour créer un token d'accès Gitea et le stocker en toute sécurité dans un dossier .gitea
g2gh -s <URL de votre instance Gitea>
# ou
g2gh --show <URL de votre instance Gitea>

# Pour migrer un dépôt Gitea vers GitHub
g2gh -r <Nom de votre Dépôt> -m <URL de votre instance Gitea> -n <Votre nom d'utilisateur Gitea>
# Ou
g2gh --repo <Nom de votre Dépôt> --migrate <URL de votre instance Gitea> --name <Votre nom d'utilisateur Gitea>

# Pour migrer tous vos dépôts Gitea vers GitHub
g2gh -a -m <URL de votre instance Gitea> -n <Votre nom d'utilisateur Gitea>
# Ou
g2gh --all --migrate <URL de votre instance Gitea> --name <Votre nom d'utilisateur Gitea>
```

### Exemples pratiques

```bash
# Migration d'un seul dépôt
g2gh -r mon-projet -m https://gitea.example.com -n mon-username

# Migration de tous les dépôts
g2gh -a -m https://gitea.example.com -n mon-username

# Affichage de l'aide
g2gh --help
```

## 🛠️ Développement

### Utilisation du script de développement

```bash
# Développement avec tests et linting
./build_and_run.sh --clean --test --lint --fmt

# Test rapide en mode debug
./build_and_run.sh

# Build optimisé sans exécution
./build_and_run.sh --release --no-run

# Passer des arguments à l'application
./build_and_run.sh --release -- -a -m https://gitea.example.com -n username
```

### Variables d'environnement

```bash
# Niveau de log détaillé
RUST_LOG=debug ./build_and_run.sh

# Affichage des stack traces
RUST_BACKTRACE=1 ./build_and_run.sh
```

## 🔒 Sécurité

- Les tokens sont gérés de manière sécurisée
- Les mots de passe sont masqués lors de la saisie
- Les informations sensibles ne sont jamais enregistrées en clair
- Vérifiez toujours les permissions de vos tokens avant utilisation

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
├── build_and_run.sh    # Script de build et d'installation automatisé
├── Cargo.toml          # Configuration Rust
└── README.md           # Documentation
```

## 📚 Dépendances Principales

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

### Workflow de développement recommandé

```bash
# Cloner votre fork
git clone https://github.com/votre-username/gitea-to-github-migrator.git
cd gitea-to-github-migrator

# Développer avec le script automatisé
./build_and_run.sh --clean --test --lint --fmt

# Tester vos changements
./build_and_run.sh --release --no-run
```

## 🗑️ Désinstallation

```bash
# Avec le script automatisé
./build_and_run.sh --uninstall

# Ou manuellement
sudo rm /usr/local/bin/g2gh
```

## 📝 Licence

Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus de détails.

## ⚠️ Notes importantes

- Vérifiez vos permissions sur les deux plateformes avant de commencer
- Utilisez des **tokens GitHub classiques**, les fine-grained tokens ne sont pas supportés
- Testez d'abord sur un petit dépôt avant de migrer des projets importants
- Le script `build_and_run.sh` est compatible Linux, macOS et Windows (Git Bash/WSL)
- Les tokens doivent avoir les bonnes permissions (voir section Configuration des tokens)

## 🚨 Résolution des problèmes courants

### Erreur 404 Not Found
- Vérifiez que l'utilisateur Gitea existe
- Confirmez l'URL de votre instance Gitea
- Testez l'accès à l'API manuellement : `curl "https://votre-gitea.com/api/v1/users/username"`

### Erreur d'authentification
- Vérifiez les permissions de vos tokens
- Assurez-vous d'utiliser des tokens classiques GitHub
- Confirmez que les tokens ne sont pas expirés

### Problèmes d'installation
- Vérifiez que Rust et Git sont installés
- Assurez-vous d'avoir les permissions sudo (Linux/macOS)
- Sur Windows, utilisez Git Bash ou WSL pour le script