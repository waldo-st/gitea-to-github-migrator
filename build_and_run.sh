#!/bin/bash

# Script de build et d'exécution pour gitea-to-github-migrator
# Compatible avec Linux, macOS et Windows (Git Bash/WSL)

set -e  # Arrêter le script en cas d'erreur

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Fonction pour afficher les messages colorés
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Fonction pour détecter l'OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "Linux";;
        Darwin*)    echo "macOS";;
        CYGWIN*)    echo "Windows";;
        MINGW*)     echo "Windows";;
        MSYS*)      echo "Windows";;
        *)          echo "Unknown";;
    esac
}

# Fonction pour vérifier si Rust est installé
check_rust() {
    if ! command -v rustc &> /dev/null; then
        print_error "Rust n'est pas installé!"
        print_info "Installez Rust depuis: https://rustup.rs/"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo n'est pas installé!"
        exit 1
    fi
    
    print_success "Rust $(rustc --version) détecté"
    print_success "Cargo $(cargo --version) détecté"
}

# Fonction pour vérifier si Git est installé
check_git() {
    if ! command -v git &> /dev/null; then
        print_error "Git n'est pas installé!"
        print_info "Installez Git depuis: https://git-scm.com/"
        exit 1
    fi
    
    print_success "Git $(git --version) détecté"
}

# Fonction pour nettoyer les builds précédents
clean_build() {
    if [ "$1" = "--clean" ] || [ "$1" = "-c" ]; then
        print_info "Nettoyage des builds précédents..."
        cargo clean
        print_success "Nettoyage terminé"
    fi
}

# Fonction pour builder le projet
build_project() {
    local build_type="$1"
    
    print_info "Construction du projet..."
    
    if [ "$build_type" = "release" ] || [ "$build_type" = "-r" ]; then
        print_info "Build en mode release..."
        cargo build --release
        BINARY_PATH="target/release/gitea-to-github-migrator"
    else
        print_info "Build en mode debug..."
        cargo build
        BINARY_PATH="target/debug/gitea-to-github-migrator"
    fi
    
    # Ajout de l'extension .exe sur Windows
    OS=$(detect_os)
    if [ "$OS" = "Windows" ]; then
        BINARY_PATH="${BINARY_PATH}.exe"
    fi
    
    if [ -f "$BINARY_PATH" ]; then
        print_success "Build réussi: $BINARY_PATH"
    else
        print_error "Build échoué: binaire non trouvé"
        exit 1
    fi
}

# Fonction pour exécuter les tests
run_tests() {
    if [ "$1" = "--test" ] || [ "$1" = "-t" ]; then
        print_info "Exécution des tests..."
        cargo test
        print_success "Tests terminés"
    fi
}

# Fonction pour linter le code
run_clippy() {
    if [ "$1" = "--lint" ] || [ "$1" = "-l" ]; then
        print_info "Analyse du code avec Clippy..."
        if command -v cargo-clippy &> /dev/null; then
            cargo clippy -- -D warnings
            print_success "Lint terminé"
        else
            print_warning "Clippy n'est pas installé. Installation..."
            rustup component add clippy
            cargo clippy -- -D warnings
            print_success "Lint terminé"
        fi
    fi
}

# Fonction pour formater le code
format_code() {
    if [ "$1" = "--fmt" ] || [ "$1" = "-f" ]; then
        print_info "Formatage du code..."
        cargo fmt
        print_success "Formatage terminé"
    fi
}

# Fonction pour installer le binaire globalement
install_binary() {
    if [ "$1" = "--install" ] || [ "$1" = "-i" ]; then
        local OS=$(detect_os)
        local install_path
        local binary_name="g2gh"
        
        # Déterminer le chemin d'installation selon l'OS
        case "$OS" in
            "Linux"|"macOS")
                install_path="/usr/local/bin"
                ;;
            "Windows")
                # Sur Windows, utiliser un répertoire dans PATH ou créer un script batch
                install_path="$HOME/bin"
                mkdir -p "$install_path"
                binary_name="g2gh.exe"
                print_warning "Sur Windows, assurez-vous que $install_path est dans votre PATH"
                ;;
            *)
                print_error "OS non supporté pour l'installation globale"
                return 1
                ;;
        esac
        
        print_info "Installation de l'outil en tant que '$binary_name'..."
        
        # Vérifier que le binaire existe
        if [ ! -f "$BINARY_PATH" ]; then
            print_error "Binaire non trouvé: $BINARY_PATH"
            print_info "Buildez d'abord le projet avec --release"
            return 1
        fi
        
        # Installation selon l'OS
        if [ "$OS" = "Windows" ]; then
            # Sur Windows, copier le binaire
            cp "$BINARY_PATH" "$install_path/$binary_name"
            chmod +x "$install_path/$binary_name"
        else
            # Sur Linux/macOS, créer un lien symbolique
            if [ -L "$install_path/$binary_name" ] || [ -f "$install_path/$binary_name" ]; then
                print_warning "Suppression de l'installation précédente..."
                sudo rm -f "$install_path/$binary_name"
            fi
            
            sudo ln -s "$(pwd)/$BINARY_PATH" "$install_path/$binary_name"
            sudo chmod 755 "$install_path/$binary_name"
        fi
        
        # Vérifier l'installation
        if command -v "$binary_name" &> /dev/null; then
            print_success "✨ Installation réussie! Vous pouvez maintenant utiliser '$binary_name' depuis n'importe où"
            print_info "Test: $binary_name --help"
        else
            print_error "Installation échouée ou '$binary_name' n'est pas dans le PATH"
            if [ "$OS" = "Windows" ]; then
                print_info "Ajoutez '$install_path' à votre PATH Windows"
            fi
        fi
        
        return 0
    fi
}

# Fonction pour désinstaller le binaire
uninstall_binary() {
    if [ "$1" = "--uninstall" ] || [ "$1" = "-u" ]; then
        local OS=$(detect_os)
        local install_path
        local binary_name="g2gh"
        
        case "$OS" in
            "Linux"|"macOS")
                install_path="/usr/local/bin"
                ;;
            "Windows")
                install_path="$HOME/bin"
                binary_name="g2gh.exe"
                ;;
            *)
                print_error "OS non supporté pour la désinstallation"
                return 1
                ;;
        esac
        
        print_info "Désinstallation de '$binary_name'..."
        
        if [ -f "$install_path/$binary_name" ] || [ -L "$install_path/$binary_name" ]; then
            if [ "$OS" = "Windows" ]; then
                rm -f "$install_path/$binary_name"
            else
                sudo rm -f "$install_path/$binary_name"
            fi
            print_success "Désinstallation réussie"
        else
            print_warning "'$binary_name' n'était pas installé"
        fi
        
        return 0
    fi
}

# Fonction pour exécuter le binaire
run_binary() {
    local skip_run=false
    
    # Vérifier si on doit skip l'exécution
    for arg in "$@"; do
        if [ "$arg" = "--no-run" ] || [ "$arg" = "-n" ] || [ "$arg" = "--install" ] || [ "$arg" = "-i" ] || [ "$arg" = "--uninstall" ] || [ "$arg" = "-u" ]; then
            skip_run=true
            break
        fi
    done
    
    if [ "$skip_run" = true ]; then
        # Vérifier si c'est à cause de --install, dans ce cas ne pas afficher le message
        local has_install=false
        for arg in "$@"; do
            if [ "$arg" = "--install" ] || [ "$arg" = "-i" ]; then
                has_install=true
                break
            fi
        done
        
        if [ "$has_install" = false ]; then
            print_info "Exécution skippée (--no-run spécifié)"
        fi
        return 0
    fi
    
    # Collecter les arguments pour l'application
    app_args=()
    skip_next=false
    
    for arg in "$@"; do
        if [ "$skip_next" = true ]; then
            skip_next=false
            continue
        fi
        
        case "$arg" in
            --clean|-c|--release|-r|--test|-t|--lint|-l|--fmt|-f|--no-run|-n|--install|-i|--uninstall|-u)
                # Ces arguments sont pour ce script, pas pour l'app
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                app_args+=("$arg")
                ;;
        esac
    done
    
    print_info "Exécution du programme..."
    print_info "Commande: ./$BINARY_PATH ${app_args[*]}"
    
    # Exécuter le binaire avec les arguments
    if [ ${#app_args[@]} -eq 0 ]; then
        "./$BINARY_PATH"
    else
        "./$BINARY_PATH" "${app_args[@]}"
    fi
}

# Fonction d'aide
show_help() {
    cat << EOF
Usage: $0 [OPTIONS] [-- APP_ARGS]

Options du script de build:
  -c, --clean       Nettoyer avant de builder
  -r, --release     Builder en mode release
  -t, --test        Exécuter les tests
  -l, --lint        Linter le code avec Clippy
  -f, --fmt         Formater le code
  -n, --no-run      Ne pas exécuter après le build
  -i, --install     Installer l'outil globalement comme 'g2gh'
  -u, --uninstall   Désinstaller l'outil global
  -h, --help        Afficher cette aide

Examples:
  $0                            # Build debug et exécute
  $0 --release                  # Build release et exécute
  $0 --clean --test --lint      # Nettoie, teste, linte, build et exécute
  $0 --release --install        # Build release et installe globalement
  $0 --uninstall                # Désinstalle l'outil global
  $0 --no-run                   # Build seulement
  $0 -- --help                  # Build et exécute avec --help
  $0 --release -- --source-url https://gitea.example.com --target-token ghp_xxx

Installation globale:
  Une fois installé avec --install, vous pourrez utiliser 'g2gh' depuis n'importe où:
  g2gh --source-url https://gitea.example.com --target-token ghp_xxx

Environment Variables:
  RUST_LOG        Niveau de log (debug, info, warn, error)
  RUST_BACKTRACE  Affichage des stack traces (1 ou full)

OS détecté: $(detect_os)
EOF
}

# Fonction principale
main() {
    print_info "🚀 Script de build et d'exécution pour gitea-to-github-migrator"
    print_info "OS détecté: $(detect_os)"
    
    # Vérifier les prérequis
    check_rust
    check_git
    
    # Traitement des arguments
    for arg in "$@"; do
        case "$arg" in
            --help|-h)
                show_help
                exit 0
                ;;
            --uninstall|-u)
                uninstall_binary "$arg"
                exit 0
                ;;
        esac
    done
    
    # Vérifier qu'on est dans le bon répertoire
    if [ ! -f "Cargo.toml" ]; then
        print_error "Cargo.toml non trouvé. Êtes-vous dans le répertoire du projet?"
        exit 1
    fi
    
    # Exécuter les étapes selon les arguments
    clean_build "$@"
    format_code "$@"
    run_clippy "$@"
    run_tests "$@"
    
    # Déterminer le mode de build
    build_mode="debug"
    for arg in "$@"; do
        if [ "$arg" = "--release" ] || [ "$arg" = "-r" ]; then
            build_mode="release"
            break
        fi
    done
    
    build_project "$build_mode"
    
    # Vérifier si l'installation est demandée
    for arg in "$@"; do
        if [ "$arg" = "--install" ] || [ "$arg" = "-i" ]; then
            install_binary "$arg"
            break
        fi
    done
    
    run_binary "$@"
    
    print_success "🎉 Terminé!"
}

# Exécuter le script principal avec tous les arguments
main "$@"