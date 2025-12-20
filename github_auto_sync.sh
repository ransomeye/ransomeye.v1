#!/bin/bash
# Path and File Name : /home/ransomeye/rebuild/github_auto_sync.sh
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Automatic GitHub synchronization script that checks for changes and pushes to remote repository

set -e

PROJECT_ROOT="/home/ransomeye/rebuild"
LOG_FILE="$PROJECT_ROOT/logs/github_sync.log"
LOCK_FILE="/tmp/ransomeye_github_sync.lock"
MAX_LOCK_AGE=3600  # 1 hour in seconds

# Ensure logs directory exists
mkdir -p "$(dirname "$LOG_FILE")"

# Logging function
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

# Cleanup function
cleanup() {
    rm -f "$LOCK_FILE"
}

# Trap to cleanup on exit
trap cleanup EXIT INT TERM

# Check for existing lock file
if [ -f "$LOCK_FILE" ]; then
    LOCK_AGE=$(($(date +%s) - $(stat -c %Y "$LOCK_FILE" 2>/dev/null || echo 0)))
    if [ "$LOCK_AGE" -lt "$MAX_LOCK_AGE" ]; then
        log "WARNING: Sync already in progress (lock file exists, age: ${LOCK_AGE}s). Skipping."
        exit 0
    else
        log "WARNING: Stale lock file detected (age: ${LOCK_AGE}s). Removing and continuing."
        rm -f "$LOCK_FILE"
    fi
fi

# Create lock file
touch "$LOCK_FILE"
echo $$ > "$LOCK_FILE"

log "=========================================="
log "Starting GitHub auto-sync"
log "=========================================="

cd "$PROJECT_ROOT"

# Check if git repository is initialized
if [ ! -d ".git" ]; then
    log "ERROR: Not a git repository. Initializing..."
    git init
    git branch -M main
    log "Git repository initialized"
fi

# Check if remote is configured
if ! git remote get-url origin &>/dev/null; then
    log "ERROR: No remote 'origin' configured. Please run sync_to_github.sh first."
    exit 1
fi

# Fetch latest changes from remote
log "Fetching latest changes from remote..."
if ! git fetch origin main 2>&1 | tee -a "$LOG_FILE"; then
    log "WARNING: Failed to fetch from remote. Continuing with local check..."
fi

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    log "Uncommitted changes detected. Staging all changes..."
    git add -A
    
    # Create commit
    COMMIT_MSG="Auto-sync: $(date '+%Y-%m-%d %H:%M:%S') - Automatic synchronization"
    if git commit -m "$COMMIT_MSG" 2>&1 | tee -a "$LOG_FILE"; then
        log "Changes committed successfully"
    else
        log "WARNING: Commit failed or no changes to commit"
    fi
fi

# Check if local branch is ahead of remote
LOCAL_COMMITS=$(git rev-list --count origin/main..HEAD 2>/dev/null || echo "0")
if [ "$LOCAL_COMMITS" -gt 0 ]; then
    log "Local branch is $LOCAL_COMMITS commit(s) ahead of remote. Pushing..."
    
    # Attempt push with credential helper
    # Ensure credential helper is available
    export GIT_TERMINAL_PROMPT=0
    export GIT_ASKPASS=""
    
    if git push origin main 2>&1 | tee -a "$LOG_FILE"; then
        log "✓ Successfully pushed to GitHub"
        log "Repository: $(git remote get-url origin)"
    else
        PUSH_ERROR=$(git push origin main 2>&1)
        echo "$PUSH_ERROR" | tee -a "$LOG_FILE"
        
        # Check if it's an authentication error
        if echo "$PUSH_ERROR" | grep -q "could not read Username\|Authentication failed\|fatal: could not read"; then
            log "ERROR: Authentication failed. Credentials may be incorrect."
            log "HINT: Run './setup_git_credentials.sh' to reconfigure"
            log "HINT: Or check ~/.git-credentials file format"
        else
            log "ERROR: Failed to push to GitHub. Check network connection."
        fi
        exit 1
    fi
else
    log "No new commits to push. Repository is up to date."
fi

# Check for remote changes that need pulling
REMOTE_COMMITS=$(git rev-list --count HEAD..origin/main 2>/dev/null || echo "0")
if [ "$REMOTE_COMMITS" -gt 0 ]; then
    log "WARNING: Remote is $REMOTE_COMMITS commit(s) ahead of local."
    log "Consider running 'git pull origin main' to sync remote changes."
fi

log "=========================================="
log "GitHub auto-sync completed"
log "=========================================="
log ""

exit 0

