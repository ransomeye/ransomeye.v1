#!/bin/bash
# Path and File Name : /home/ransomeye/rebuild/setup_auto_sync.sh
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Setup script for automatic GitHub synchronization

set -e

PROJECT_ROOT="/home/ransomeye/rebuild"
SYSTEMD_DIR="$PROJECT_ROOT/systemd"
SERVICE_NAME="ransomeye-github-sync"
TIMER_NAME="ransomeye-github-sync.timer"

echo "=========================================="
echo "RansomEye GitHub Auto-Sync Setup"
echo "=========================================="
echo ""

# Check if running as root for systemd operations
if [ "$EUID" -ne 0 ]; then
    echo "This script needs sudo privileges to install systemd services."
    echo "Please run: sudo $0"
    exit 1
fi

# Verify files exist
if [ ! -f "$PROJECT_ROOT/github_auto_sync.sh" ]; then
    echo "ERROR: github_auto_sync.sh not found!"
    exit 1
fi

if [ ! -f "$SYSTEMD_DIR/${SERVICE_NAME}.service" ]; then
    echo "ERROR: ${SERVICE_NAME}.service not found!"
    exit 1
fi

if [ ! -f "$SYSTEMD_DIR/${TIMER_NAME}" ]; then
    echo "ERROR: ${TIMER_NAME} not found!"
    exit 1
fi

# Copy systemd files
echo "Installing systemd service and timer..."
cp "$SYSTEMD_DIR/${SERVICE_NAME}.service" /etc/systemd/system/
cp "$SYSTEMD_DIR/${TIMER_NAME}" /etc/systemd/system/

# Reload systemd
echo "Reloading systemd daemon..."
systemctl daemon-reload

# Enable and start timer
echo "Enabling and starting timer..."
systemctl enable "${TIMER_NAME}"
systemctl start "${TIMER_NAME}"

# Show status
echo ""
echo "=========================================="
echo "Auto-sync Status"
echo "=========================================="
systemctl status "${TIMER_NAME}" --no-pager -l || true

echo ""
echo "=========================================="
echo "✓ Auto-sync setup complete!"
echo "=========================================="
echo ""
echo "The repository will sync to GitHub every hour automatically."
echo ""
echo "Useful commands:"
echo "  Check timer status:  sudo systemctl status ${TIMER_NAME}"
echo "  Check service logs:  sudo journalctl -u ${SERVICE_NAME}.service -f"
echo "  Manually trigger:    sudo systemctl start ${SERVICE_NAME}.service"
echo "  Disable auto-sync:   sudo systemctl stop ${TIMER_NAME}"
echo "  Enable auto-sync:    sudo systemctl start ${TIMER_NAME}"
echo ""
echo "Log file: $PROJECT_ROOT/logs/github_sync.log"
echo ""

