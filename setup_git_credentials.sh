#!/bin/bash
# Path and File Name : /home/ransomeye/rebuild/setup_git_credentials.sh
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Setup Git credential helper for automatic authentication

set -e

echo "=========================================="
echo "Git Credential Helper Setup"
echo "=========================================="
echo ""
echo "This script will help you configure Git credentials for automatic authentication."
echo ""

# Check current user
CURRENT_USER=$(whoami)
echo "Current user: $CURRENT_USER"
echo ""

# Option 1: Credential Store (Simple, less secure)
echo "Option 1: Credential Store (credentials stored in plain text)"
echo "  - Simple and works for most cases"
echo "  - Credentials stored in ~/.git-credentials"
echo ""

# Option 2: SSH (Most secure, recommended)
echo "Option 2: SSH Authentication (Recommended)"
echo "  - Most secure method"
echo "  - Requires SSH key setup"
echo ""

read -p "Choose method (1=store, 2=ssh, 3=skip): " METHOD

case $METHOD in
    1)
        echo ""
        echo "Setting up credential store..."
        git config --global credential.helper store
        
        echo ""
        echo "Now you need to provide your GitHub credentials."
        echo "You can either:"
        echo "  A) Push manually once (git push) and credentials will be saved"
        echo "  B) Create ~/.git-credentials file manually"
        echo ""
        read -p "Do you want to create credentials file now? (y/n): " CREATE_NOW
        
        if [ "$CREATE_NOW" = "y" ]; then
            echo ""
            read -p "GitHub username: " GITHUB_USER
            read -sp "GitHub Personal Access Token: " GITHUB_TOKEN
            echo ""
            
            # Create credentials file
            CRED_FILE="$HOME/.git-credentials"
            echo "https://${GITHUB_USER}:${GITHUB_TOKEN}@github.com" > "$CRED_FILE"
            chmod 600 "$CRED_FILE"
            
            echo "✓ Credentials saved to $CRED_FILE"
            echo "  File permissions set to 600 (read/write for owner only)"
        else
            echo ""
            echo "To set up credentials later, run:"
            echo "  git push origin main"
            echo "  (Enter your username and Personal Access Token when prompted)"
        fi
        ;;
    2)
        echo ""
        echo "Setting up SSH authentication..."
        
        # Check if SSH key exists
        if [ -f "$HOME/.ssh/id_ed25519" ] || [ -f "$HOME/.ssh/id_rsa" ]; then
            echo "✓ SSH key found"
            if [ -f "$HOME/.ssh/id_ed25519.pub" ]; then
                SSH_KEY="$HOME/.ssh/id_ed25519.pub"
            elif [ -f "$HOME/.ssh/id_rsa.pub" ]; then
                SSH_KEY="$HOME/.ssh/id_rsa.pub"
            fi
            
            echo ""
            echo "Your public SSH key:"
            cat "$SSH_KEY"
            echo ""
            echo "Add this key to GitHub: https://github.com/settings/keys"
            echo "Click 'New SSH key', paste the key above, and save."
            echo ""
            read -p "Press Enter after adding the key to GitHub..."
        else
            echo "No SSH key found. Generating new SSH key..."
            read -p "Email for SSH key (default: gagan@ransomeye.tech): " SSH_EMAIL
            SSH_EMAIL=${SSH_EMAIL:-gagan@ransomeye.tech}
            
            ssh-keygen -t ed25519 -C "$SSH_EMAIL" -f "$HOME/.ssh/id_ed25519" -N ""
            
            echo ""
            echo "Your public SSH key:"
            cat "$HOME/.ssh/id_ed25519.pub"
            echo ""
            echo "Add this key to GitHub: https://github.com/settings/keys"
            echo "Click 'New SSH key', paste the key above, and save."
            echo ""
            read -p "Press Enter after adding the key to GitHub..."
        fi
        
        # Update remote to use SSH
        cd /home/ransomeye/rebuild
        CURRENT_REMOTE=$(git remote get-url origin 2>/dev/null || echo "")
        if [[ "$CURRENT_REMOTE" == https://* ]]; then
            # Convert HTTPS to SSH
            REPO_PATH=$(echo "$CURRENT_REMOTE" | sed 's|https://github.com/||' | sed 's|\.git$||')
            git remote set-url origin "git@github.com:${REPO_PATH}.git"
            echo "✓ Updated remote URL to use SSH"
        fi
        
        # Test SSH connection
        echo ""
        echo "Testing SSH connection to GitHub..."
        if ssh -T git@github.com 2>&1 | grep -q "successfully authenticated"; then
            echo "✓ SSH authentication successful!"
        else
            echo "⚠ SSH test completed (this is normal)"
        fi
        ;;
    3)
        echo "Skipping credential setup."
        echo "You'll need to authenticate manually for each push."
        exit 0
        ;;
    *)
        echo "Invalid option. Exiting."
        exit 1
        ;;
esac

echo ""
echo "=========================================="
echo "✓ Credential setup complete!"
echo "=========================================="
echo ""
echo "You can now test with:"
echo "  cd /home/ransomeye/rebuild"
echo "  git push origin main"
echo ""

