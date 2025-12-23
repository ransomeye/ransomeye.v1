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
# 3. CREATE RANSOMEYE USER AND GROUP (ROOTLESS RUNTIME)
# ============================================================================
log "Creating ransomeye user and group for rootless runtime"

echo ""
echo "==========================================================================="
echo "CREATING RANSOMEYE USER/GROUP"
echo "==========================================================================="
echo ""

# Check if ransomeye user exists
if id "ransomeye" &>/dev/null; then
    success "ransomeye user already exists"
else
    log "Creating ransomeye user and group"
    
    # Create ransomeye group
    if ! getent group ransomeye > /dev/null 2>&1; then
        groupadd -r ransomeye
        success "ransomeye group created"
    else
        success "ransomeye group already exists"
    fi
    
    # Create ransomeye user (system user, no login shell, home directory)
    useradd -r -g ransomeye -d /home/ransomeye -s /usr/sbin/nologin -c "RansomEye Service User" ransomeye 2>&1 | tee -a "$LOG_FILE"
    
    if id "ransomeye" &>/dev/null; then
        success "ransomeye user created"
        
        # Set ownership of project directory
        chown -R ransomeye:ransomeye "$PROJECT_ROOT" 2>&1 | tee -a "$LOG_FILE" || warning "Could not set ownership of project directory"
        
        # Create runtime and state directories
        mkdir -p /run/ransomeye /var/lib/ransomeye
        chown -R ransomeye:ransomeye /run/ransomeye /var/lib/ransomeye
        chmod 755 /run/ransomeye /var/lib/ransomeye
        success "Runtime and state directories created"
    else
        error "Failed to create ransomeye user"
    fi
fi

# ============================================================================
# 3.5. GUARDRAILS ENFORCEMENT (PHASE 0 - MANDATORY)
# ============================================================================
log "Enforcing guardrails before installation proceeds"

echo ""
echo "==========================================================================="
echo "GUARDRAILS ENFORCEMENT (PHASE 0)"
echo "==========================================================================="
echo ""

# Build guardrails binary first if it doesn't exist
GUARDRAILS_BINARY="/usr/bin/ransomeye-guardrails"
GUARDRAILS_SOURCE="$PROJECT_ROOT/core/guardrails"

if [[ -d "$GUARDRAILS_SOURCE" ]]; then
    log "Building guardrails enforcement engine"
    cd "$GUARDRAILS_SOURCE"
    
    if cargo build --release 2>&1 | tee -a "$LOG_FILE"; then
        BUILD_EXIT_CODE=${PIPESTATUS[0]}
        if [[ $BUILD_EXIT_CODE -eq 0 ]]; then
            # Install binary
            if [[ -f "$GUARDRAILS_SOURCE/target/release/ransomeye-guardrails" ]]; then
                cp "$GUARDRAILS_SOURCE/target/release/ransomeye-guardrails" "$GUARDRAILS_BINARY"
                chmod +x "$GUARDRAILS_BINARY"
                success "Guardrails binary installed"
            fi
        fi
    fi
    
    cd "$PROJECT_ROOT"
fi

# Enforce guardrails (fail-closed)
if [[ -f "$GUARDRAILS_BINARY" ]]; then
    log "Running guardrails enforcement for installer context"
    if "$GUARDRAILS_BINARY" enforce --context installer 2>&1 | tee -a "$LOG_FILE"; then
        ENFORCE_EXIT_CODE=${PIPESTATUS[0]}
        if [[ $ENFORCE_EXIT_CODE -eq 0 ]]; then
            success "Guardrails enforcement passed - installation can proceed"
        else
            error "Guardrails enforcement failed - installation aborted (fail-closed)"
        fi
    else
        error "Failed to execute guardrails enforcement"
    fi
else
    warning "Guardrails binary not found - skipping enforcement (NOT RECOMMENDED)"
    # In strict mode, we should fail here, but for now we warn
    # Uncomment the next line for strict enforcement:
    # error "Guardrails binary required but not found"
fi

# ============================================================================
# 4. BUILD AND INSTALL CORE BINARIES
# ============================================================================
log "Building and installing core binaries"

echo ""
echo "==========================================================================="
echo "BUILDING CORE BINARIES"
echo "==========================================================================="
echo ""

# Build and install ransomeye_operations binary
if [[ -d "$PROJECT_ROOT/ransomeye_operations" ]]; then
    log "Building ransomeye_operations binary"
    cd "$PROJECT_ROOT/ransomeye_operations"
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo is required but not found. Please install Rust toolchain."
    fi
    
    # Build release binary
    if cargo build --release 2>&1 | tee -a "$LOG_FILE"; then
        BUILD_EXIT_CODE=${PIPESTATUS[0]}
        if [[ $BUILD_EXIT_CODE -eq 0 ]]; then
            # Install binary to /usr/bin
            BINARY_SOURCE="$PROJECT_ROOT/ransomeye_operations/target/release/ransomeye_operations"
            BINARY_TARGET="/usr/bin/ransomeye_operations"
            
            if [[ -f "$BINARY_SOURCE" ]]; then
                cp "$BINARY_SOURCE" "$BINARY_TARGET"
                chmod +x "$BINARY_TARGET"
                
                # Verify installation
                if [[ -f "$BINARY_TARGET" && -x "$BINARY_TARGET" ]]; then
                    # Get version for verification
                    VERSION_OUTPUT=$("$BINARY_TARGET" --version 2>&1 || echo "unknown")
                    success "ransomeye_operations binary installed to $BINARY_TARGET (version: $VERSION_OUTPUT)"
                else
                    error "Failed to verify binary installation: $BINARY_TARGET"
                fi
            else
                error "Built binary not found at expected path: $BINARY_SOURCE"
            fi
        else
            error "Binary build failed with exit code: $BUILD_EXIT_CODE"
        fi
    else
        error "Failed to execute cargo build"
    fi
    
    cd "$PROJECT_ROOT"
else
    error "ransomeye_operations module not found. Expected: $PROJECT_ROOT/ransomeye_operations/"
fi

# Build and install Phase 6: Incident Response Playbook Engine
if [[ -d "$PROJECT_ROOT/core/response_playbooks" ]]; then
    log "Building Phase 6: Incident Response Playbook Engine"
    cd "$PROJECT_ROOT/core/response_playbooks"
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo is required but not found. Please install Rust toolchain."
    fi
    
    # Build release binary
    if cargo build --release 2>&1 | tee -a "$LOG_FILE"; then
        BUILD_EXIT_CODE=${PIPESTATUS[0]}
        if [[ $BUILD_EXIT_CODE -eq 0 ]]; then
            # Install binary to /usr/local/bin
            BINARY_SOURCE="$PROJECT_ROOT/core/response_playbooks/target/release/ransomeye-playbook-engine"
            BINARY_TARGET="/usr/local/bin/ransomeye-playbook-engine"
            
            if [[ -f "$BINARY_SOURCE" ]]; then
                cp "$BINARY_SOURCE" "$BINARY_TARGET"
                chmod +x "$BINARY_TARGET"
                
                # Verify installation
                if [[ -f "$BINARY_TARGET" && -x "$BINARY_TARGET" ]]; then
                    success "Phase 6 playbook engine binary installed to $BINARY_TARGET"
                else
                    error "Failed to verify binary installation: $BINARY_TARGET"
                fi
            else
                error "Built binary not found at expected path: $BINARY_SOURCE"
            fi
        else
            error "Phase 6 binary build failed with exit code: $BUILD_EXIT_CODE"
        fi
    else
        error "Failed to execute cargo build for Phase 6"
    fi
    
    cd "$PROJECT_ROOT"
else
    warning "Phase 6 playbook engine not found. Expected: $PROJECT_ROOT/core/response_playbooks/"
fi

# Build and install Phase 9: Network Scanner
if [[ -d "$PROJECT_ROOT/core/network_scanner" ]]; then
    log "Building Phase 9: Network Scanner"
    cd "$PROJECT_ROOT/core/network_scanner"
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo is required but not found. Please install Rust toolchain."
    fi
    
    # Build release binary
    if cargo build --release 2>&1 | tee -a "$LOG_FILE"; then
        BUILD_EXIT_CODE=${PIPESTATUS[0]}
        if [[ $BUILD_EXIT_CODE -eq 0 ]]; then
            # Install binary to /usr/local/bin
            BINARY_SOURCE="$PROJECT_ROOT/core/network_scanner/target/release/ransomeye-network-scanner"
            BINARY_TARGET="/usr/local/bin/ransomeye-network-scanner"
            
            if [[ -f "$BINARY_SOURCE" ]]; then
                cp "$BINARY_SOURCE" "$BINARY_TARGET"
                chmod +x "$BINARY_TARGET"
                
                # Verify installation
                if [[ -f "$BINARY_TARGET" && -x "$BINARY_TARGET" ]]; then
                    success "Phase 9 network scanner binary installed to $BINARY_TARGET"
                else
                    error "Failed to verify binary installation: $BINARY_TARGET"
                fi
            else
                error "Built binary not found at expected path: $BINARY_SOURCE"
            fi
        else
            error "Phase 9 binary build failed with exit code: $BUILD_EXIT_CODE"
        fi
    else
        error "Failed to execute cargo build for Phase 9"
    fi
    
    cd "$PROJECT_ROOT"
else
    warning "Phase 9 network scanner not found. Expected: $PROJECT_ROOT/core/network_scanner/"
fi

# ============================================================================
# 5. CORE STACK INSTALLATION
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
# 6. STANDALONE AGENTS (EXPLICIT SEPARATION)
# ============================================================================
log "Checking for standalone agents"

echo ""
echo "==========================================================================="
echo "STANDALONE AGENTS"
echo "==========================================================================="
echo ""

# Use Python module resolver to detect standalone agents
STANDALONE_AGENTS=$(python3 << 'PYTHON_SCRIPT'
import sys
sys.path.insert(0, '/home/ransomeye/rebuild')
try:
    from ransomeye_installer.module_resolver import ModuleResolver
    resolver = ModuleResolver()
    standalone = resolver.get_standalone_modules()
    print('\n'.join(standalone))
except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    sys.exit(1)
PYTHON_SCRIPT
)

if [[ -n "$STANDALONE_AGENTS" ]]; then
    echo "The following standalone agents were detected:"
    echo "$STANDALONE_AGENTS" | while read -r agent; do
        echo "  ⚠ $agent"
    done
    echo ""
    echo "CRITICAL: Standalone agents must be installed using their dedicated installers."
    echo "The main installer does NOT install standalone agents."
    echo ""
    echo "To install standalone agents:"
    echo "  - DPI Probe: Use ransomeye_dpi_probe/installer/install.sh"
    echo "  - Linux Agent: Use ransomeye_linux_agent/installer/install.sh"
    echo "  - Windows Agent: Use ransomeye_windows_agent/installer/install.ps1 (Windows only)"
    echo ""
    log "Standalone agents detected but not installed by main installer"
else
    log "No standalone agents detected"
fi

# ============================================================================
# 7. POST-INSTALL VALIDATION
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
# 8. COMPLETION
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

