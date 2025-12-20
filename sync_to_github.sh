#!/bin/bash
# Path and File Name : /home/ransomeye/rebuild/sync_to_github.sh
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Script to sync RansomEye project to GitHub repository

set -e

PROJECT_ROOT="/home/ransomeye/rebuild"
REPO_NAME="ransomeye-rebuild"

echo "=========================================="
echo "RansomEye GitHub Sync Script"
echo "=========================================="
echo ""

# Check if git is installed
if ! command -v git &> /dev/null; then
    echo "ERROR: Git is not installed. Please install git first."
    exit 1
fi

# Check if GitHub CLI is installed (optional but helpful)
if command -v gh &> /dev/null; then
    echo "✓ GitHub CLI (gh) is installed"
    USE_GH_CLI=true
else
    echo "⚠ GitHub CLI (gh) is not installed. Will use manual method."
    USE_GH_CLI=false
fi

# Install Git LFS if not installed
if ! git lfs version &> /dev/null; then
    echo "Installing Git LFS..."
    if command -v apt-get &> /dev/null; then
        sudo apt-get update && sudo apt-get install -y git-lfs
    elif command -v yum &> /dev/null; then
        sudo yum install -y git-lfs
    elif command -v brew &> /dev/null; then
        brew install git-lfs
    else
        echo "ERROR: Cannot install Git LFS automatically. Please install it manually."
        echo "Visit: https://git-lfs.github.com/"
        exit 1
    fi
fi

# Initialize Git LFS
echo "Setting up Git LFS..."
git lfs install

# Configure Git LFS to track large files
echo "Configuring Git LFS file patterns..."
git lfs track "*.pkl"
git lfs track "*.gguf"
git lfs track "*.h5"
git lfs track "*.pb"
git lfs track "*.onnx"
git lfs track "*.pt"
git lfs track "*.pth"
git lfs track "*.ckpt"
git lfs track "*.safetensors"
git lfs track "datasets/**/*.csv"
git lfs track "datasets/**/*.parquet"
git lfs track "datasets/**/*.feather"
git lfs track "models/**/*"

# Configure git user if not set
if [ -z "$(git config user.name)" ]; then
    echo ""
    echo "Git user configuration is required."
    read -p "Enter your Git username: " GIT_USERNAME
    git config user.name "$GIT_USERNAME"
fi

if [ -z "$(git config user.email)" ]; then
    echo ""
    read -p "Enter your Git email: " GIT_EMAIL
    git config user.email "$GIT_EMAIL"
fi

echo ""
echo "Current Git configuration:"
echo "  User: $(git config user.name)"
echo "  Email: $(git config user.email)"
echo ""

# Check if already a git repo
if [ ! -d "$PROJECT_ROOT/.git" ]; then
    echo "Initializing Git repository..."
    cd "$PROJECT_ROOT"
    git init
    git branch -M main
fi

# Add .gitattributes for LFS
if [ ! -f "$PROJECT_ROOT/.gitattributes" ]; then
    echo "Creating .gitattributes file..."
    git lfs track "*.pkl"
    git lfs track "*.gguf"
    git lfs track "*.h5"
    git lfs track "*.pb"
    git lfs track "*.onnx"
    git lfs track "*.pt"
    git lfs track "*.pth"
    git lfs track "*.ckpt"
    git lfs track "*.safetensors"
fi

# Stage all files
echo ""
echo "Staging all files..."
cd "$PROJECT_ROOT"
git add .gitignore
git add .gitattributes 2>/dev/null || true
git add .

# Check repository size
echo ""
echo "Checking repository size..."
REPO_SIZE=$(du -sh .git 2>/dev/null | cut -f1 || echo "unknown")
echo "Repository size: $REPO_SIZE"

# Create initial commit
if [ -z "$(git log --oneline 2>/dev/null)" ]; then
    echo ""
    echo "Creating initial commit..."
    git commit -m "Initial commit: RansomEye Enterprise Security Platform

- Complete RansomEye rebuild project
- All 23 phases included
- Guardrails, retention, and trust modules
- Trained models and datasets (via Git LFS)
- Full compliance with enterprise standards"
fi

# GitHub repository creation
echo ""
echo "=========================================="
echo "GitHub Repository Setup"
echo "=========================================="
echo ""

if [ "$USE_GH_CLI" = true ]; then
    # Check if authenticated
    if gh auth status &> /dev/null; then
        echo "✓ GitHub CLI is authenticated"
        
        # Check if repo exists
        if gh repo view "$REPO_NAME" &> /dev/null; then
            echo "Repository '$REPO_NAME' already exists."
            read -p "Do you want to push to existing repository? (y/n): " PUSH_EXISTING
            if [ "$PUSH_EXISTING" = "y" ]; then
                REMOTE_URL=$(gh repo view "$REPO_NAME" --json url -q .url)
            else
                read -p "Enter new repository name: " REPO_NAME
                echo "Creating new repository: $REPO_NAME"
                gh repo create "$REPO_NAME" --public --source=. --remote=origin --push
                echo "✓ Repository created and code pushed!"
                exit 0
            fi
        else
            echo "Creating new repository: $REPO_NAME"
            read -p "Make repository private? (y/n): " IS_PRIVATE
            if [ "$IS_PRIVATE" = "y" ]; then
                gh repo create "$REPO_NAME" --private --source=. --remote=origin --push
            else
                gh repo create "$REPO_NAME" --public --source=. --remote=origin --push
            fi
            echo "✓ Repository created and code pushed!"
            exit 0
        fi
    else
        echo "GitHub CLI is not authenticated."
        echo "Please run: gh auth login"
        echo ""
        read -p "Press Enter to continue with manual setup..."
    fi
fi

# Manual GitHub setup
echo ""
echo "Manual GitHub Repository Setup:"
echo "1. Go to https://github.com/new"
echo "2. Create a new repository named: $REPO_NAME"
echo "3. DO NOT initialize with README, .gitignore, or license"
echo "4. Copy the repository URL"
echo ""
read -p "Enter your GitHub repository URL (e.g., https://github.com/username/ransomeye-rebuild.git): " REPO_URL

if [ -z "$REPO_URL" ]; then
    echo "ERROR: Repository URL is required."
    exit 1
fi

# Add remote
echo ""
echo "Adding remote repository..."
git remote remove origin 2>/dev/null || true
git remote add origin "$REPO_URL"

# Push to GitHub
echo ""
echo "Pushing to GitHub..."
echo "This may take a while if you have large files..."
git push -u origin main

echo ""
echo "=========================================="
echo "✓ Successfully synced to GitHub!"
echo "Repository: $REPO_URL"
echo "=========================================="

