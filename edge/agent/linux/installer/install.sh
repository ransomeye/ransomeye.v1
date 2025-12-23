#!/bin/bash
# Path and File Name: /home/ransomeye/rebuild/ransomeye_linux_agent/installer/install.sh
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details: Linux Agent standalone installer - enforces EULA, validates NO-SWAP requirement, installs service

set -euo pipefail

set -o errexit
set -o nounset
set -o pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MODULE_DIR="$(dirname "$SCRIPT_DIR")"
INSTALL_DIR="/opt/ransomeye/linux_agent"
SYSTEMD_DIR="/etc/systemd/system"
LOG_FILE="/var/log/ransomeye/linux_agent_install.log"
RECEIPT_FILE="$INSTALL_DIR/.install_receipt.json"
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

log "Starting RansomEye Linux Agent installation"

# Check root privileges
if [[ $EUID -ne 0 ]]; then
    error "This installer must be run as root (use sudo)"
fi

# ============================================================================
# 1. EULA ENFORCEMENT (MANDATORY, NO BYPASS)
# ============================================================================
EULA_FILE="$SCRIPT_DIR/EULA.txt"
if [[ ! -f "$EULA_FILE" ]]; then
    error "EULA file not found at $EULA_FILE"
fi

echo ""
echo "==========================================================================="
echo "END USER LICENSE AGREEMENT (EULA)"
echo "==========================================================================="
if [[ -s "$EULA_FILE" ]]; then
    cat "$EULA_FILE"
else
    echo "EULA terms apply. Please contact support@ransomeye.tech for details."
fi
echo "==========================================================================="
echo ""

while true; do
    read -p "Do you accept the EULA? (yes/no): " eula_response
    case "$eula_response" in
        yes|YES|y|Y)
            success "EULA accepted"
            break
            ;;
        no|NO|n|N)
            error "EULA not accepted. Installation aborted."
            ;;
        *)
            echo "Please enter 'yes' or 'no'"
            ;;
    esac
done

# ============================================================================
# 2. BINARY SIGNATURE VERIFICATION
# ============================================================================
BINARY_PATH="$MODULE_DIR/target/release/ransomeye_linux_agent"
if [[ ! -f "$BINARY_PATH" ]]; then
    error "Binary not found at $BINARY_PATH. Please build the project first: cargo build --release"
fi

if [[ ! -x "$BINARY_PATH" ]]; then
    error "Binary is not executable: $BINARY_PATH"
fi

# Verify binary signature
if command -v gpg &> /dev/null; then
    SIG_FILE="${BINARY_PATH}.sig"
    if [[ -f "$SIG_FILE" ]]; then
        if gpg --verify "$SIG_FILE" "$BINARY_PATH" 2>&1 | tee -a "$LOG_FILE"; then
            success "Binary signature verified"
        else
            error "Binary signature verification failed"
        fi
    else
        warning "Signature file not found, skipping verification"
    fi
else
    warning "GPG not available, skipping signature verification"
fi

# ============================================================================
# 3. ENVIRONMENT CONSTRAINTS
# ============================================================================

# OS check (Linux required)
if [[ "$(uname)" != "Linux" ]]; then
    error "Linux Agent requires Linux. Detected: $(uname)"
fi
success "OS check passed: $(uname -a)"

# ============================================================================
# 4. EXPLICIT NO-SWAP VALIDATION (MANDATORY - FAIL IF SWAP EXISTS)
# ============================================================================
log "Validating NO-SWAP requirement..."

CURRENT_SWAP_GB=$(free -g | awk '/^Swap:/ {print $2}')

if [[ $CURRENT_SWAP_GB -gt 0 ]]; then
    error "FAIL: Swap detected (${CURRENT_SWAP_GB}GB). Linux Agent MUST NOT use swap. Please disable all swap before installation."
fi

# Check for swap files
if swapon --show | grep -q .; then
    error "FAIL: Active swap detected. Linux Agent MUST NOT use swap. Please disable all swap: swapoff -a"
fi

# Check /etc/fstab for swap entries
if grep -q "swap" /etc/fstab 2>/dev/null; then
    warning "Swap entries found in /etc/fstab. These will be ignored if swap is disabled."
    read -p "Continue anyway? (yes/no): " continue_with_fstab
    if [[ "$continue_with_fstab" != "yes" ]]; then
        error "Installation aborted due to swap configuration in /etc/fstab"
    fi
fi

success "NO-SWAP validation passed: No swap detected or enabled"

# ============================================================================
# 5. PRIVILEGE DOWNGRADE PREPARATION
# ============================================================================
log "Preparing privilege downgrade..."

# Create dedicated user/group if they don't exist
if ! id -u "$RUN_USER" &> /dev/null; then
    useradd --system --no-create-home --shell /bin/false "$RUN_USER"
    success "Created user: $RUN_USER"
else
    success "User exists: $RUN_USER"
fi

if ! getent group "$RUN_GROUP" &> /dev/null; then
    groupadd --system "$RUN_GROUP"
    success "Created group: $RUN_GROUP"
else
    success "Group exists: $RUN_GROUP"
fi

# Ensure user is in the group
usermod -a -G "$RUN_GROUP" "$RUN_USER" 2>/dev/null || true

# ============================================================================
# 6. INSTALLATION
# ============================================================================

# Create installation directory
mkdir -p "$INSTALL_DIR"/{bin,config,logs}
success "Installation directory created: $INSTALL_DIR"

# Copy binary
cp "$BINARY_PATH" "$INSTALL_DIR/bin/"
chmod +x "$INSTALL_DIR/bin/ransomeye_linux_agent"
success "Binary installed"

# Set ownership (will be changed to RUN_USER after privilege downgrade)
chown root:root "$INSTALL_DIR/bin/ransomeye_linux_agent"
chmod 4750 "$INSTALL_DIR/bin/ransomeye_linux_agent"  # SetUID for privilege downgrade

# Create default config directory structure
mkdir -p "$INSTALL_DIR/config"
if [[ -d "$MODULE_DIR/config" ]]; then
    cp -r "$MODULE_DIR/config"/* "$INSTALL_DIR/config/" 2>/dev/null || true
fi

# Set directory ownership
chown -R "$RUN_USER:$RUN_GROUP" "$INSTALL_DIR/config" "$INSTALL_DIR/logs"
chmod 755 "$INSTALL_DIR"
chmod 750 "$INSTALL_DIR/config" "$INSTALL_DIR/logs"

# ============================================================================
# 7. SYSTEMD SERVICE INSTALLATION
# ============================================================================
SERVICE_FILE="$MODULE_DIR/systemd/ransomeye-linux-agent.service"
if [[ ! -f "$SERVICE_FILE" ]]; then
    error "Service file not found: $SERVICE_FILE"
fi

cp "$SERVICE_FILE" "$SYSTEMD_DIR/"
systemctl daemon-reload
success "Systemd service installed"

# ============================================================================
# 8. INSTALL RECEIPT (SIGNED)
# ============================================================================
INSTALL_TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
INSTALL_USER=$(whoami)
BINARY_HASH=$(sha256sum "$INSTALL_DIR/bin/ransomeye_linux_agent" | cut -d' ' -f1)

cat > "$RECEIPT_FILE" <<EOF
{
  "module": "ransomeye_linux_agent",
  "version": "1.0.0",
  "install_timestamp": "$INSTALL_TIMESTAMP",
  "install_user": "$INSTALL_USER",
  "install_dir": "$INSTALL_DIR",
  "binary_hash": "$BINARY_HASH",
  "eula_accepted": true,
  "swap_detected": false,
  "run_user": "$RUN_USER",
  "run_group": "$RUN_GROUP",
  "kernel_version": "$(uname -r)"
}
EOF

# Sign receipt (if GPG available)
if command -v gpg &> /dev/null; then
    if gpg --default-key "$(gpg --list-keys --keyid-format LONG | head -3 | tail -1 | awk '{print $2}' | cut -d'/' -f2)" --detach-sign --armor "$RECEIPT_FILE" 2>&1 | tee -a "$LOG_FILE"; then
        success "Install receipt signed"
    else
        warning "Failed to sign install receipt"
    fi
fi

chmod 600 "$RECEIPT_FILE"
success "Install receipt created: $RECEIPT_FILE"

# ============================================================================
# 9. COMPLETION
# ============================================================================
log "Installation completed successfully"

echo ""
echo "==========================================================================="
echo "Installation Summary"
echo "==========================================================================="
echo "Module:     RansomEye Linux Agent"
echo "Version:    1.0.0"
echo "Install:    $INSTALL_DIR"
echo "Service:    ransomeye-linux-agent.service"
echo "Run User:   $RUN_USER"
echo "Swap:       NONE (validated)"
echo "==========================================================================="
echo ""
echo "Next steps:"
echo "  1. Configure environment variables in: $INSTALL_DIR/config/"
echo "  2. Start service: systemctl start ransomeye-linux-agent"
echo "  3. Enable auto-start: systemctl enable ransomeye-linux-agent"
echo "  4. Verify installation: $SCRIPT_DIR/verify.sh"
echo ""

success "Installation complete"

