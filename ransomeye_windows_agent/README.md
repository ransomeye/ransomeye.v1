# RansomEye Windows Agent

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_windows_agent/README.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Stand-alone Windows Agent sensor for endpoint telemetry collection

## Overview

The Windows Agent is a **stand-alone, untrusted sensor** that collects endpoint telemetry. It never enforces policy, never runs AI, and never stores long-term state.

## Responsibilities

- Process creation telemetry
- Registry telemetry
- File activity telemetry
- Controlled response hooks (NOT enforcement)

## Hard Rules

- No kernel drivers (user-mode only)
- No policy enforcement
- No AI
- Signed telemetry only

## Components

- **Telemetry Collector**: Collects process, registry, and file events (user-mode)
- **Event Signer**: Signs all telemetry with RSA-4096-PSS-SHA256
- **Transport Client**: mTLS client for sending signed events to Core
- **Backpressure Handler**: Manages bounded buffers and backpressure signals
- **Health Reporter**: Reports sensor health status

## Configuration

All configuration via environment variables:

- `CORE_API_URL`: Core API endpoint (default: `https://localhost:8443`)
- `BUFFER_DIR`: Buffer directory (default: `C:\ProgramData\RansomEye\WindowsAgent\buffer`)
- `AGENT_CERT_PATH`: Client certificate path
- `AGENT_KEY_PATH`: Client private key path
- `CA_CERT_PATH`: CA certificate path
- `MAX_BUFFER_SIZE_MB`: Maximum buffer size in MB (default: 512)
- `BACKPRESSURE_THRESHOLD`: Backpressure threshold (default: 4096)
- `TELEMETRY_INTERVAL_SECONDS`: Telemetry collection interval (default: 1)

## Communication

- mTLS authentication with per-instance certificates
- Event signing with RSA-4096-PSS-SHA256
- Replay protection via nonce
- Backpressure handling

## Testing

Run integration tests:

```bash
cd ransomeye_windows_agent
cargo test
```

Tests cover:
- Identity spoofing prevention
- Event signing verification
- Backpressure handling
- Core unavailability tolerance
- Resource exhaustion handling

## Installation

### Prerequisites

- Windows 10 (Build 1809+) or Windows Server 2019+
- Administrator privileges
- WiX Toolset v3.11+ (for building MSI installer)
- Windows SDK (for code signing)

### Build MSI Installer

1. Build the binary:
```bash
cd ransomeye_windows_agent
cargo build --release
```

2. Build the MSI installer:
```powershell
.\installer\build-msi.ps1
```

This requires WiX Toolset installed. Download from: https://wixtoolset.org/

3. Sign the MSI (recommended):
```powershell
signtool sign /f certificate.pfx /p password .\installer\RansomEyeWindowsAgent.msi
```

### Install

**Option 1: PowerShell wrapper (recommended)**
```powershell
.\installer\install.ps1 -MsiPath .\installer\RansomEyeWindowsAgent.msi -AcceptEULA
```

**Option 2: Direct MSI installation**
```powershell
msiexec.exe /i .\installer\RansomEyeWindowsAgent.msi /quiet ACCEPTEULA=1
```

**Option 3: Via Control Panel**
1. Double-click `RansomEyeWindowsAgent.msi`
2. Follow the installation wizard
3. Accept EULA when prompted

The installer will:
- Enforce EULA acceptance (mandatory, no bypass)
- Verify Authenticode signatures (if signed)
- Install binary to `C:\Program Files\RansomEye\WindowsAgent\`
- Install data/config to `C:\ProgramData\RansomEye\WindowsAgent\`
- Install and start Windows Service
- Create install receipt

### Service Management

**Start service:**
```powershell
Start-Service -Name "RansomEyeWindowsAgent"
```

**Stop service:**
```powershell
Stop-Service -Name "RansomEyeWindowsAgent"
```

**Restart service:**
```powershell
Restart-Service -Name "RansomEyeWindowsAgent"
```

**View service status:**
```powershell
Get-Service -Name "RansomEyeWindowsAgent"
```

**View logs:**
```powershell
# Event Viewer
eventvwr.msc

# Application logs
Get-Content "C:\ProgramData\RansomEye\WindowsAgent\logs\*.log" -Tail 50
```

See `installer/lifecycle.md` for complete lifecycle management documentation.

### Uninstallation

**Option 1: PowerShell wrapper**
```powershell
.\installer\uninstall.ps1 [-PreserveLogs]
```

**Option 2: Control Panel**
1. Open "Add or Remove Programs" (Settings → Apps)
2. Find "RansomEye Windows Agent"
3. Click "Uninstall"

**Option 3: MSI command line**
```powershell
msiexec.exe /x {ProductCode} /quiet
```

The uninstaller will:
- Stop and remove the Windows Service
- Remove binaries and configuration
- Optionally preserve logs (configurable)
- Clean registry entries

### Verify Installation

```powershell
.\installer\verify.ps1
```

## Requirements

See `installer/requirements.md` for detailed system requirements, including:
- OS and architecture requirements
- Memory and disk requirements
- Privilege requirements
- Build and signing requirements

## Build

```bash
cargo build --release
```

## Run (Manual)

For manual execution (not recommended for production):

```powershell
.\target\release\ransomeye_windows_agent.exe
```

**Note:** Production deployments should use the Windows Service installed via the MSI installer.

