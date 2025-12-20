#!/bin/bash
# Path and File Name : /home/ransomeye/rebuild/fix_credentials.sh
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Fix Git credentials format for GitHub authentication

set -e

echo "=========================================="
echo "Git Credentials Fix"
echo "=========================================="
echo ""

CRED_FILE="$HOME/.git-credentials"

if [ ! -f "$CRED_FILE" ]; then
    echo "ERROR: Credentials file not found: $CRED_FILE"
    echo "Please run: ./setup_git_credentials.sh"
    exit 1
fi

echo "Current credentials file:"
cat "$CRED_FILE" | sed 's/\(https:\/\/[^:]*:\).*\(@.*\)/\1***\2/'
echo ""

echo "The credentials file should have format:"
echo "  https://GITHUB_USERNAME:TOKEN@github.com"
echo ""
echo "Note: Use your GitHub USERNAME (not email) for Personal Access Tokens"
echo ""

read -p "Enter your GitHub username (not email): " GITHUB_USER
read -sp "Enter your GitHub Personal Access Token: " GITHUB_TOKEN
echo ""

# Create credentials file with correct format
echo "https://${GITHUB_USER}:${GITHUB_TOKEN}@github.com" > "$CRED_FILE"
chmod 600 "$CRED_FILE"

echo "✓ Credentials updated"
echo ""

# Test the credentials
echo "Testing credentials..."
cd /home/ransomeye/rebuild
export GIT_TERMINAL_PROMPT=0
export GIT_ASKPASS=""

if git ls-remote origin main &>/dev/null; then
    echo "✓ Credentials are working!"
else
    echo "⚠ Could not verify credentials. Please check:"
    echo "  1. GitHub username is correct (not email)"
    echo "  2. Personal Access Token has 'repo' scope"
    echo "  3. Token is not expired"
fi

echo ""
echo "You can now test with:"
echo "  git push origin main"
echo ""

