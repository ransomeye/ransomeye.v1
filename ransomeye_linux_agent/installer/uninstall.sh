#!/bin/bash
# Path and File Name: /home/ransomeye/rebuild/ransomeye_linux_agent/installer/uninstall.sh
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details: Linux Agent standalone uninstaller - clean removal with configurable log preservation

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="/opt/ransomeye/linux_agent"
SYSTEMD_DIR="/etc/systemd/system"
SERVICE_NAME="ransomeye-linux-agent.service"
LOG_FILE="/var/log/ransomeye/linux_agent_uninstall.log"
RUN_USER="ransomeye"
RUN_GROUP="ransomeye"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

error() {
    echo -e "${RED}ERROR: $1${NC}" | tee -a "$LOG_FILE" >&2
    exit 1
}

success() {
    echo -e "${GREEN}✓ $1${NC}" | tee -a "$LOG_FILE"
}

warning() {
    echo -e "${YELLOW}⚠ $1${NC}" | tee -a "$LOG_FILE"
}

mkdir -p "$(dirname "$LOG_FILE")"

log "Starting RansomEye Linux Agent uninstallation"

# Check root
if [[ $EUID -ne 0 ]]; then
    error "This uninstaller must be run as root (use sudo)"
fi

# Confirmation
echo ""
echo "This will uninstall RansomEye Linux Agent."
echo "WARNING: This will stop the service and remove all installed files."
echo ""

PRESERVE_LOGS="yes"
read -p "Preserve logs and configuration? (yes/no) [yes]: " preserve_response
if [[ -n "$preserve_response" ]]; then
    PRESERVE_LOGS="$preserve_response"
fi

read -p "Are you sure you want to proceed? (yes/no): " confirm
if [[ "$confirm" != "yes" ]]; then
    log "Uninstallation cancelled by user"
    exit 0
fi

# ============================================================================
# 1. STOP SERVICE
# ============================================================================
log "Stopping service..."

if systemctl is-active --quiet "$SERVICE_NAME" 2>/dev/null; then
    systemctl stop "$SERVICE_NAME"
    success "Service stopped"
else
    warning "Service was not running"
fi

if systemctl is-enabled --quiet "$SERVICE_NAME" 2>/dev/null; then
    systemctl disable "$SERVICE_NAME"
    success "Service disabled"
fi

# ============================================================================
# 2. REMOVE SYSTEMD SERVICE
# ============================================================================
log "Removing systemd service..."

if [[ -f "$SYSTEMD_DIR/$SERVICE_NAME" ]]; then
    rm -f "$SYSTEMD_DIR/$SERVICE_NAME"
    systemctl daemon-reload
    success "Systemd service removed"
else
    warning "Service file not found: $SYSTEMD_DIR/$SERVICE_NAME"
fi

# ============================================================================
# 3. REMOVE BINARIES
# ============================================================================
log "Removing binaries..."

if [[ -d "$INSTALL_DIR" ]]; then
    if [[ "$PRESERVE_LOGS" == "yes" ]]; then
        rm -rf "$INSTALL_DIR/bin"
        success "Binaries removed (logs and config preserved)"
    else
        rm -rf "$INSTALL_DIR"
        success "Installation directory removed"
    fi
else
    warning "Installation directory not found: $INSTALL_DIR"
fi

# ============================================================================
# 4. USER/GROUP REMOVAL (OPTIONAL)
# ============================================================================
log "Checking for user/group removal..."

read -p "Remove user '$RUN_USER' and group '$RUN_GROUP'? (yes/no) [no]: " remove_user
if [[ "$remove_user" == "yes" ]]; then
    if id -u "$RUN_USER" &> /dev/null; then
        userdel "$RUN_USER" 2>/dev/null || warning "Failed to remove user (may be in use)"
    fi
    if getent group "$RUN_GROUP" &> /dev/null; then
        groupdel "$RUN_GROUP" 2>/dev/null || warning "Failed to remove group (may be in use)"
    fi
    success "User and group removed"
else
    success "User and group preserved"
fi

# ============================================================================
# 5. CLEANUP
# ============================================================================
log "Cleaning up..."

if [[ -d "$(dirname "$INSTALL_DIR")" ]] && [[ -z "$(ls -A "$(dirname "$INSTALL_DIR")" 2>/dev/null)" ]]; then
    rmdir "$(dirname "$INSTALL_DIR")" 2>/dev/null || true
fi

success "Cleanup complete"

# ============================================================================
# 6. COMPLETION
# ============================================================================
log "Uninstallation completed"

echo ""
echo "==========================================================================="
echo "Uninstallation Summary"
echo "==========================================================================="
echo "Service:    Removed and disabled"
echo "Binaries:   Removed"
if [[ "$PRESERVE_LOGS" == "yes" ]]; then
    echo "Logs:       Preserved at $INSTALL_DIR"
fi
echo "==========================================================================="
echo ""

success "Uninstallation complete"

