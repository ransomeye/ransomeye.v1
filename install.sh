#!/bin/bash
# Path and File Name: /home/ransomeye/rebuild/install.sh
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details: Root-level installation entrypoint - ONLY supported installation mechanism for RansomEye

set -euo pipefail

# Fail-closed: exit immediately on any error
set -o errexit
set -o nounset
set -o pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR"
EULA_PATH="$PROJECT_ROOT/ransomeye_installer/eula/EULA.txt"
LOG_FILE="/var/log/ransomeye/install.log"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE" 2>/dev/null || echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
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

# Ensure log directory exists
mkdir -p "$(dirname "$LOG_FILE")" 2>/dev/null || true

log "Starting RansomEye installation"

# ============================================================================
# 1. ROOT PRIVILEGE CHECK (MANDATORY)
# ============================================================================
if [[ $EUID -ne 0 ]]; then
    error "This installer MUST be run as root. Please use: sudo ./install.sh"
fi

success "Root privileges confirmed"

# ============================================================================
# 2. GLOBAL EULA ENFORCEMENT (MANDATORY, NO BYPASS)
# ============================================================================
log "Displaying global EULA"

if [[ ! -f "$EULA_PATH" ]]; then
    error "Global EULA file not found at: $EULA_PATH"
fi

echo ""
echo "==========================================================================="
echo "RANSOMEYE - END USER LICENSE AGREEMENT (EULA)"
echo "==========================================================================="
echo ""

if [[ -s "$EULA_PATH" ]]; then
    cat "$EULA_PATH"
else
    echo "END USER LICENSE AGREEMENT"
    echo ""
    echo "By installing RansomEye, you agree to the following terms:"
    echo ""
    echo "1. RansomEye is proprietary software owned by RansomEye.Tech"
    echo "2. Use is subject to license terms provided separately"
    echo "3. Support: Gagan@RansomEye.Tech"
    echo "4. © RansomEye.Tech"
    echo ""
fi

echo "==========================================================================="
echo ""

while true; do
    read -p "Do you accept the EULA? (yes/no): " eula_response
    case "$eula_response" in
        yes|YES|y|Y)
            success "Global EULA accepted"
            EULA_ACCEPTED=true
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
# 3. CORE STACK INSTALLATION
# ============================================================================
log "Installing RansomEye core stack"

echo ""
echo "==========================================================================="
echo "CORE STACK INSTALLATION"
echo "==========================================================================="
echo ""

# Check if Python installer module exists
if [[ -d "$PROJECT_ROOT/ransomeye_installer" ]]; then
    # Use Python installer
    log "Using Python installer (ransomeye_installer)"
    
    # Change to project root for Python module import
    cd "$PROJECT_ROOT"
    
    # Run Python installer (EULA already accepted, but installer will re-display)
    # The installer will handle its own EULA display - we accept it here
    if python3 -m ransomeye_installer.installer 2>&1 | tee -a "$LOG_FILE"; then
        INSTALLER_EXIT_CODE=${PIPESTATUS[0]}
        if [[ $INSTALLER_EXIT_CODE -eq 0 ]]; then
            success "Core stack installation completed"
        else
            error "Core stack installation failed with exit code: $INSTALLER_EXIT_CODE"
        fi
    else
        error "Failed to execute Python installer"
    fi
else
    error "RansomEye installer not found. Expected: $PROJECT_ROOT/ransomeye_installer/"
fi

# ============================================================================
# 4. OPTIONAL STANDALONE MODULES
# ============================================================================
log "Checking for optional standalone modules"

echo ""
echo "==========================================================================="
echo "OPTIONAL STANDALONE MODULES"
echo "==========================================================================="
echo ""
echo "The following standalone modules can be installed:"
echo "  1. DPI Probe (Phase 23) - Network packet inspection"
echo "  2. Linux Agent (Phase 21) - Host telemetry collection"
echo "  3. Windows Agent (Phase 22) - Endpoint telemetry (Windows only)"
echo ""

read -p "Install standalone modules? (yes/no) [no]: " install_standalone
INSTALL_STANDALONE=${install_standalone:-no}

if [[ "$INSTALL_STANDALONE" == "yes" ]] || [[ "$INSTALL_STANDALONE" == "YES" ]] || [[ "$INSTALL_STANDALONE" == "y" ]] || [[ "$INSTALL_STANDALONE" == "Y" ]]; then
    log "Installing standalone modules"
    
    # DPI Probe
    if [[ -f "$PROJECT_ROOT/ransomeye_dpi_probe/installer/install.sh" ]]; then
        echo ""
        read -p "Install DPI Probe? (yes/no) [no]: " install_dpi
        if [[ "$install_dpi" == "yes" ]] || [[ "$install_dpi" == "YES" ]] || [[ "$install_dpi" == "y" ]] || [[ "$install_dpi" == "Y" ]]; then
            log "Installing DPI Probe"
            if bash "$PROJECT_ROOT/ransomeye_dpi_probe/installer/install.sh" 2>&1 | tee -a "$LOG_FILE"; then
                success "DPI Probe installed"
            else
                warning "DPI Probe installation failed or was cancelled"
            fi
        fi
    fi
    
    # Linux Agent
    if [[ -f "$PROJECT_ROOT/ransomeye_linux_agent/installer/install.sh" ]]; then
        echo ""
        read -p "Install Linux Agent? (yes/no) [no]: " install_linux
        if [[ "$install_linux" == "yes" ]] || [[ "$install_linux" == "YES" ]] || [[ "$install_linux" == "y" ]] || [[ "$install_linux" == "Y" ]]; then
            log "Installing Linux Agent"
            if bash "$PROJECT_ROOT/ransomeye_linux_agent/installer/install.sh" 2>&1 | tee -a "$LOG_FILE"; then
                success "Linux Agent installed"
            else
                warning "Linux Agent installation failed or was cancelled"
            fi
        fi
    fi
    
    # Windows Agent (Linux system won't install this, but check anyway)
    if [[ -f "$PROJECT_ROOT/ransomeye_windows_agent/installer/install.ps1" ]] && command -v pwsh &> /dev/null; then
        echo ""
        warning "Windows Agent installer found, but this is a Linux system. Windows Agent should be installed on Windows systems."
    fi
else
    log "Skipping standalone modules"
fi

# ============================================================================
# 5. POST-INSTALL VALIDATION
# ============================================================================
log "Running post-install validation"

echo ""
echo "==========================================================================="
echo "POST-INSTALL VALIDATION"
echo "==========================================================================="
echo ""

VALIDATOR_PATH="$PROJECT_ROOT/post_install_validator.py"

if [[ -f "$VALIDATOR_PATH" ]]; then
    # Run validator as root (we're already root)
    if python3 "$VALIDATOR_PATH" 2>&1 | tee -a "$LOG_FILE"; then
        VALIDATOR_EXIT_CODE=${PIPESTATUS[0]}
        if [[ $VALIDATOR_EXIT_CODE -eq 0 ]]; then
            success "Post-install validation passed"
        else
            error "Post-install validation FAILED with exit code: $VALIDATOR_EXIT_CODE"
        fi
    else
        error "Failed to execute post-install validator"
    fi
else
    warning "Post-install validator not found: $VALIDATOR_PATH"
    warning "Installation completed but validation was skipped"
fi

# ============================================================================
# 6. COMPLETION
# ============================================================================
log "Installation process completed"

echo ""
echo "==========================================================================="
echo "INSTALLATION COMPLETE"
echo "==========================================================================="
echo ""
echo "RansomEye has been installed successfully."
echo ""
echo "Installation log: $LOG_FILE"
echo ""
echo "Next steps:"
echo "  1. Review service configurations in: $PROJECT_ROOT/systemd/"
echo "  2. Configure environment variables as needed"
echo "  3. Enable services: sudo systemctl enable ransomeye-*"
echo "  4. Start services: sudo systemctl start ransomeye-*"
echo "  5. Check status: sudo systemctl status ransomeye-*"
echo ""
echo "For uninstallation: sudo ./uninstall.sh"
echo "==========================================================================="
echo ""

success "Installation completed successfully"

exit 0

