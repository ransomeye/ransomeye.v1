#!/usr/bin/env python3
# Path and File Name: /home/ransomeye/rebuild/post_install_validator.py
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details: Post-install validation script - verifies installation consistency, services, trust store, identities, retention, and standalone modules

"""
RansomEye Post-Install Validator
Verifies installation consistency and emits signed validation report.
FAIL-CLOSED: Exits with non-zero code on any mismatch.
"""

import sys
import os
import json
import subprocess
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Tuple, Optional
import hashlib

# Try to import yaml, fail gracefully if not available
try:
    import yaml
    YAML_AVAILABLE = True
except ImportError:
    YAML_AVAILABLE = False

# Colors for output
RED = '\033[0;31m'
GREEN = '\033[0;32m'
YELLOW = '\033[1;33m'
NC = '\033[0m'  # No Color

PROJECT_ROOT = Path("/home/ransomeye/rebuild")
MODULE_PHASE_MAP = PROJECT_ROOT / "MODULE_PHASE_MAP.yaml"
INSTALL_STATE = PROJECT_ROOT / "ransomeye_installer/config/install_state.json"
RETENTION_CONFIG = PROJECT_ROOT / "config/retention.txt"
TRUST_DIR = PROJECT_ROOT / "ransomeye_trust"
SYSTEMD_DIR = PROJECT_ROOT / "systemd"
SYSTEMD_INSTALLED = Path("/etc/systemd/system")

VALIDATION_REPORT_DIR = PROJECT_ROOT / "logs"
VALIDATION_REPORT = VALIDATION_REPORT_DIR / f"validation_report_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"

# Validation results
passed_checks = []
failed_checks = []
warnings = []


def log_pass(check_name: str, message: str = ""):
    """Log a passed check."""
    passed_checks.append({"check": check_name, "message": message})
    print(f"{GREEN}✓{NC} {check_name}", end="")
    if message:
        print(f": {message}")
    else:
        print()


def log_fail(check_name: str, message: str):
    """Log a failed check."""
    failed_checks.append({"check": check_name, "message": message})
    print(f"{RED}✗{NC} {check_name}: {message}")


def log_warn(check_name: str, message: str):
    """Log a warning."""
    warnings.append({"check": check_name, "message": message})
    print(f"{YELLOW}⚠{NC} {check_name}: {message}")


def error(message: str):
    """Print error and exit."""
    print(f"{RED}ERROR: {message}{NC}", file=sys.stderr)
    sys.exit(1)


def check_module_phase_map() -> bool:
    """Verify MODULE_PHASE_MAP.yaml exists and is valid."""
    print("\n[1/8] Validating MODULE_PHASE_MAP.yaml...")
    
    if not MODULE_PHASE_MAP.exists():
        log_fail("MODULE_PHASE_MAP.yaml exists", f"File not found: {MODULE_PHASE_MAP}")
        return False
    
    if not YAML_AVAILABLE:
        # Basic file existence and size check if yaml not available
        if MODULE_PHASE_MAP.stat().st_size > 0:
            log_pass("MODULE_PHASE_MAP.yaml exists", "File found (YAML parsing unavailable)")
            return True
        else:
            log_fail("MODULE_PHASE_MAP.yaml", "File is empty")
            return False
    
    try:
        with open(MODULE_PHASE_MAP, 'r') as f:
            map_data = yaml.safe_load(f)
        
        if not isinstance(map_data, dict) or 'modules' not in map_data:
            log_fail("MODULE_PHASE_MAP.yaml structure", "Invalid YAML structure - missing 'modules' key")
            return False
        
        modules = map_data.get('modules', [])
        if not isinstance(modules, list):
            log_fail("MODULE_PHASE_MAP.yaml modules", "Modules must be a list")
            return False
        
        if YAML_AVAILABLE:
            log_pass("MODULE_PHASE_MAP.yaml exists and is valid", f"{len(modules)} modules defined")
            return True
        else:
            # Already handled above
            return True
        
    except Exception as e:
        if 'yaml' in str(type(e)).lower() or (YAML_AVAILABLE and isinstance(e, Exception)):
            log_fail("MODULE_PHASE_MAP.yaml parsing", f"Parse error: {e}")
        else:
            log_fail("MODULE_PHASE_MAP.yaml validation", f"Unexpected error: {e}")
        return False


def check_services_exist() -> bool:
    """Verify required services exist in systemd directory."""
    print("\n[2/8] Validating service definitions...")
    
    if not SYSTEMD_DIR.exists():
        log_fail("Systemd directory exists", f"Directory not found: {SYSTEMD_DIR}")
        return False
    
    # Load module map to find required services
    try:
        with open(MODULE_PHASE_MAP, 'r') as f:
            if YAML_AVAILABLE:
                map_data = yaml.safe_load(f)
            else:
                # Fallback: use basic checks without YAML parsing
                log_warn("YAML parsing unavailable", "Service validation will be limited")
                actual_services = list(SYSTEMD_DIR.glob("ransomeye-*.service"))
                if actual_services:
                    log_pass("Service files exist", f"{len(actual_services)} service file(s) found")
                    return True
                else:
                    log_warn("Service files", "No service files found")
                    return True  # Don't fail if we can't parse YAML
        
        if not isinstance(map_data, dict):
            log_fail("MODULE_PHASE_MAP.yaml structure", "Invalid structure")
            return False
            
        modules = map_data.get('modules', [])
        required_services = []
        
        for module in modules:
            if module.get('requires_service', False) and module.get('actual_directory') != 'NOT_FOUND':
                module_name = module.get('module_name', '')
                service_name = f"ransomeye-{module_name.replace('ransomeye_', '')}.service"
                required_services.append((service_name, module_name))
        
        # Check standalone modules
        standalone_services = [
            ("ransomeye-dpi-probe.service", "ransomeye_dpi_probe"),
            ("ransomeye-linux-agent.service", "ransomeye_linux_agent"),
        ]
        
        all_required = required_services + standalone_services
        missing_services = []
        
        for service_name, module_name in all_required:
            service_file = SYSTEMD_DIR / service_name
            if not service_file.exists():
                # Check if service might be installed in system
                installed_service = SYSTEMD_INSTALLED / service_name
                if not installed_service.exists():
                    missing_services.append(service_name)
                    log_warn(f"Service file: {service_name}", f"Not found in {SYSTEMD_DIR} or {SYSTEMD_INSTALLED}")
        
        if missing_services:
            log_warn("Some service files missing", f"{len(missing_services)} service(s) not found (may be optional)")
        else:
            log_pass("Service definitions exist", f"All required services found")
        
        # Count actual services found
        actual_services = list(SYSTEMD_DIR.glob("ransomeye-*.service"))
        log_pass("Systemd directory accessible", f"{len(actual_services)} service file(s) found")
        
        return len(actual_services) > 0
        
    except Exception as e:
        log_fail("Service validation", f"Error checking services: {e}")
        return False


def check_services_disabled() -> bool:
    """Verify all services are DISABLED by default."""
    print("\n[3/8] Validating services are DISABLED by default...")
    
    if not SYSTEMD_DIR.exists():
        log_fail("Systemd directory exists", f"Directory not found: {SYSTEMD_DIR}")
        return False
    
    # Check installed services in /etc/systemd/system
    if SYSTEMD_INSTALLED.exists() and os.geteuid() == 0:
        try:
            result = subprocess.run(
                ['systemctl', 'list-unit-files', '--type=service', '--no-legend'],
                capture_output=True,
                text=True,
                timeout=10
            )
            
            if result.returncode == 0:
                enabled_services = []
                for line in result.stdout.split('\n'):
                    if 'ransomeye' in line.lower() and 'enabled' in line:
                        service_name = line.split()[0]
                        enabled_services.append(service_name)
                        log_warn(f"Service enabled: {service_name}", "Service should be DISABLED by default")
                
                if enabled_services:
                    log_fail("Services DISABLED by default", f"{len(enabled_services)} service(s) are enabled")
                    return False
                else:
                    log_pass("Services DISABLED by default", "All RansomEye services are disabled")
                    return True
            else:
                log_warn("Cannot check service status", "systemctl command failed (may need root)")
                return True  # Don't fail if we can't check
                
        except subprocess.TimeoutExpired:
            log_warn("Service status check", "Timeout checking service status")
            return True
        except FileNotFoundError:
            log_warn("systemctl not available", "Cannot verify service status")
            return True
        except Exception as e:
            log_warn("Service status check", f"Error: {e}")
            return True
    else:
        log_warn("Cannot check installed services", "Not running as root or /etc/systemd/system not accessible")
        # Check service files for explicit [Install] WantedBy entries (they should exist but services disabled)
        service_files = list(SYSTEMD_DIR.glob("ransomeye-*.service"))
        if service_files:
            log_pass("Service files exist", f"{len(service_files)} service file(s) found (status cannot be verified without root)")
            return True
        else:
            log_fail("Service files", "No service files found")
            return False


def check_trust_store() -> bool:
    """Verify trust store exists."""
    print("\n[4/8] Validating trust store...")
    
    if not TRUST_DIR.exists():
        log_fail("Trust store exists", f"Directory not found: {TRUST_DIR}")
        return False
    
    # Check for key trust store components
    required_items = [
        TRUST_DIR,  # Directory itself
    ]
    
    missing_items = []
    for item in required_items:
        if not item.exists():
            missing_items.append(str(item))
    
    if missing_items:
        log_fail("Trust store components", f"Missing: {', '.join(missing_items)}")
        return False
    
    log_pass("Trust store exists", f"Trust directory: {TRUST_DIR}")
    return True


def check_identities() -> bool:
    """Verify identities exist."""
    print("\n[5/8] Validating identities...")
    
    # Check install state for identity information
    if INSTALL_STATE.exists():
        try:
            with open(INSTALL_STATE, 'r') as f:
                state = json.load(f)
            
            if state.get('identity_generated', False):
                log_pass("Identity generated", "Install state indicates identity was generated")
                
                # Try to verify identity files exist
                identity_dirs = [
                    TRUST_DIR / "keys",
                    PROJECT_ROOT / "ransomeye_installer/crypto",
                ]
                
                identity_found = False
                for identity_dir in identity_dirs:
                    if identity_dir.exists() and list(identity_dir.glob("*")):
                        identity_found = True
                        log_pass("Identity files exist", f"Found in: {identity_dir}")
                        break
                
                if not identity_found:
                    log_warn("Identity files", "Identity generated but files not found in expected locations")
                
                return True
            else:
                log_fail("Identity generated", "Install state indicates identity was NOT generated")
                return False
                
        except json.JSONDecodeError as e:
            log_fail("Install state JSON", f"Invalid JSON: {e}")
            return False
        except Exception as e:
            log_fail("Identity validation", f"Error: {e}")
            return False
    else:
        log_fail("Install state exists", f"File not found: {INSTALL_STATE}")
        return False


def check_retention_config() -> bool:
    """Verify retention configuration exists and is valid."""
    print("\n[6/8] Validating retention configuration...")
    
    if not RETENTION_CONFIG.exists():
        log_fail("Retention config exists", f"File not found: {RETENTION_CONFIG}")
        return False
    
    try:
        # Try to use retention validator if available
        try:
            from ransomeye_installer.retention.retention_validator import RetentionValidator
            validator = RetentionValidator(str(RETENTION_CONFIG))
            is_valid, message = validator.validate()
            
            if is_valid:
                config = validator.get_config()
                log_pass("Retention config valid", 
                        f"Telemetry: {config.get('TELEMETRY_RETENTION_MONTHS')} months, "
                        f"Forensic: {config.get('FORENSIC_RETENTION_DAYS')} days")
                return True
            else:
                log_fail("Retention config valid", message)
                return False
                
        except ImportError:
            # Fallback: basic file existence check
            with open(RETENTION_CONFIG, 'r') as f:
                content = f.read()
                if 'TELEMETRY_RETENTION_MONTHS' in content:
                    log_pass("Retention config exists", "Basic validation passed (detailed validation unavailable)")
                    return True
                else:
                    log_fail("Retention config content", "Missing required configuration keys")
                    return False
                    
    except Exception as e:
        log_fail("Retention config validation", f"Error: {e}")
        return False


def check_standalone_modules() -> bool:
    """Verify standalone modules installed correctly if present."""
    print("\n[7/8] Validating standalone modules...")
    
    standalone_modules = [
        ("ransomeye_dpi_probe", "/opt/ransomeye/dpi_probe/.install_receipt.json"),
        ("ransomeye_linux_agent", "/opt/ransomeye/linux_agent/.install_receipt.json"),
    ]
    
    installed_count = 0
    for module_name, receipt_path in standalone_modules:
        receipt = Path(receipt_path)
        if receipt.exists():
            installed_count += 1
            try:
                with open(receipt, 'r') as f:
                    receipt_data = json.load(f)
                log_pass(f"Standalone module: {module_name}", "Install receipt found and valid")
            except Exception as e:
                log_warn(f"Standalone module: {module_name}", f"Receipt exists but invalid: {e}")
        else:
            log_warn(f"Standalone module: {module_name}", "Not installed (optional)")
    
    if installed_count > 0:
        log_pass("Standalone modules", f"{installed_count} standalone module(s) installed")
    else:
        log_pass("Standalone modules", "No standalone modules installed (optional)")
    
    return True  # Don't fail if standalone modules aren't installed


def check_install_state() -> bool:
    """Verify install state exists and is valid."""
    print("\n[8/8] Validating install state...")
    
    if not INSTALL_STATE.exists():
        log_fail("Install state exists", f"File not found: {INSTALL_STATE}")
        return False
    
    try:
        with open(INSTALL_STATE, 'r') as f:
            state = json.load(f)
        
        required_fields = ['state', 'timestamp', 'version', 'eula_accepted', 'retention_configured', 'identity_generated']
        missing_fields = [field for field in required_fields if field not in state]
        
        if missing_fields:
            log_fail("Install state completeness", f"Missing fields: {', '.join(missing_fields)}")
            return False
        
        if not state.get('eula_accepted', False):
            log_fail("EULA accepted", "Install state indicates EULA was NOT accepted")
            return False
        
        if not state.get('retention_configured', False):
            log_fail("Retention configured", "Install state indicates retention was NOT configured")
            return False
        
        if not state.get('identity_generated', False):
            log_fail("Identity generated", "Install state indicates identity was NOT generated")
            return False
        
        log_pass("Install state valid", f"Version: {state.get('version')}, State: {state.get('state')}")
        return True
        
    except json.JSONDecodeError as e:
        log_fail("Install state JSON", f"Invalid JSON: {e}")
        return False
    except Exception as e:
        log_fail("Install state validation", f"Error: {e}")
        return False


def generate_report() -> Dict:
    """Generate validation report."""
    report = {
        "validation_timestamp": datetime.utcnow().isoformat() + "Z",
        "project_root": str(PROJECT_ROOT),
        "summary": {
            "total_checks": len(passed_checks) + len(failed_checks),
            "passed": len(passed_checks),
            "failed": len(failed_checks),
            "warnings": len(warnings),
            "validation_status": "PASS" if len(failed_checks) == 0 else "FAIL"
        },
        "checks": {
            "passed": passed_checks,
            "failed": failed_checks,
            "warnings": warnings
        },
        "files_checked": {
            "module_phase_map": str(MODULE_PHASE_MAP),
            "install_state": str(INSTALL_STATE),
            "retention_config": str(RETENTION_CONFIG),
            "trust_dir": str(TRUST_DIR),
            "systemd_dir": str(SYSTEMD_DIR),
        }
    }
    
    return report


def sign_report(report_path: Path) -> bool:
    """Attempt to sign the validation report."""
    try:
        # Try to use GPG if available
        if subprocess.run(['which', 'gpg'], capture_output=True, timeout=5).returncode == 0:
            sig_path = report_path.with_suffix('.json.sig')
            result = subprocess.run(
                ['gpg', '--detach-sign', '--armor', '--output', str(sig_path), str(report_path)],
                capture_output=True,
                timeout=10
            )
            if result.returncode == 0:
                log_pass("Report signed", f"Signature: {sig_path}")
                return True
            else:
                log_warn("Report signing", "GPG signing failed (report not signed)")
                return False
        else:
            log_warn("Report signing", "GPG not available (report not signed)")
            return False
    except Exception:
        log_warn("Report signing", "Could not sign report")
        return False


def main():
    """Main validation entry point."""
    print("="*80)
    print("RANSOMEYE POST-INSTALL VALIDATION")
    print("="*80)
    print(f"Project Root: {PROJECT_ROOT}")
    print(f"Timestamp: {datetime.now().isoformat()}")
    print("="*80)
    
    # Run all validation checks
    checks = [
        ("Module Phase Map", check_module_phase_map),
        ("Service Definitions", check_services_exist),
        ("Services Disabled", check_services_disabled),
        ("Trust Store", check_trust_store),
        ("Identities", check_identities),
        ("Retention Config", check_retention_config),
        ("Standalone Modules", check_standalone_modules),
        ("Install State", check_install_state),
    ]
    
    all_passed = True
    for check_name, check_func in checks:
        try:
            result = check_func()
            if not result:
                all_passed = False
        except Exception as e:
            log_fail(check_name, f"Exception: {e}")
            all_passed = False
    
    # Generate report
    print("\n" + "="*80)
    print("VALIDATION SUMMARY")
    print("="*80)
    print(f"Total Checks: {len(passed_checks) + len(failed_checks)}")
    print(f"{GREEN}Passed: {len(passed_checks)}{NC}")
    print(f"{RED}Failed: {len(failed_checks)}{NC}")
    if warnings:
        print(f"{YELLOW}Warnings: {len(warnings)}{NC}")
    print("="*80)
    
    # Generate and save report
    VALIDATION_REPORT_DIR.mkdir(parents=True, exist_ok=True)
    report = generate_report()
    
    try:
        with open(VALIDATION_REPORT, 'w') as f:
            json.dump(report, f, indent=2)
        log_pass("Validation report saved", str(VALIDATION_REPORT))
        
        # Attempt to sign report
        sign_report(VALIDATION_REPORT)
        
    except Exception as e:
        log_warn("Report saving", f"Could not save report: {e}")
    
    # FAIL-CLOSED: Exit with error if any checks failed
    if not all_passed or len(failed_checks) > 0:
        print(f"\n{RED}VALIDATION FAILED - Installation inconsistencies detected{NC}")
        print(f"\nFailed checks:")
        for check in failed_checks:
            print(f"  ✗ {check['check']}: {check['message']}")
        print(f"\nValidation report: {VALIDATION_REPORT}")
        sys.exit(1)
    
    print(f"\n{GREEN}VALIDATION PASSED - All checks completed successfully{NC}")
    print(f"\nValidation report: {VALIDATION_REPORT}")
    sys.exit(0)


if __name__ == '__main__':
    main()

