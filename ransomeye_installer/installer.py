# Path and File Name : /home/ransomeye/rebuild/ransomeye_installer/installer.py
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Main installer orchestrator - validates prerequisites, enforces EULA, configures retention, generates identity, creates systemd units

"""
RansomEye Unified Installer: Main orchestrator for installation.
Validates prerequisites, enforces EULA, configures retention, generates identity.
"""

import sys
import os
from pathlib import Path
from typing import Optional, Tuple

from .state_manager import StateManager
from .system.os_check import OSCheck
from .system.disk_check import DiskCheck
from .system.swap_check import SwapCheck
from .system.clock_check import ClockCheck
from .retention.retention_writer import RetentionWriter
from .retention.retention_validator import RetentionValidator
from .crypto.identity_generator import IdentityGenerator
from .services.systemd_writer import SystemdWriter
from .module_resolver import ModuleResolver
from .manifest_generator import ManifestGenerator


class RansomEyeInstaller:
    """Main installer orchestrator."""
    
    VERSION = "1.0.0"
    EULA_PATH = Path("/home/ransomeye/rebuild/ransomeye_installer/eula/EULA.txt")
    
    def __init__(self):
        self.state_manager = StateManager()
        self.os_check = OSCheck()
        self.disk_check = DiskCheck()
        self.swap_check = SwapCheck()
        self.clock_check = ClockCheck()
        self.retention_writer = RetentionWriter()
        self.retention_validator = RetentionValidator()
        self.identity_generator = IdentityGenerator()
        
        # Initialize module resolver (validates modules exist on disk)
        self.module_resolver = ModuleResolver()
        
        # Check for phantom modules (fail-closed)
        if self.module_resolver.phantom_modules:
            print("ERROR: Phantom modules detected:", file=sys.stderr)
            for phantom in sorted(self.module_resolver.phantom_modules):
                print(f"  ✗ {phantom}", file=sys.stderr)
            print("Installation aborted (fail-closed).", file=sys.stderr)
            sys.exit(1)
        
        # Initialize systemd writer (uses module resolver)
        self.systemd_writer = SystemdWriter()
        
        # Initialize manifest generator
        self.manifest_generator = ManifestGenerator()
    
    def _validate_prerequisites(self) -> Tuple[bool, str]:
        """
        Validate all system prerequisites.
        
        Returns:
            Tuple of (is_valid: bool, error_message: str)
        """
        # Check OS
        is_supported, os_reason = self.os_check.is_supported()
        if not is_supported:
            return False, f"OS check failed: {os_reason}"
        print(f"✓ {os_reason}")
        
        # Check disk
        is_available, disk_message, disk_usage = self.disk_check.check_availability()
        if not is_available:
            return False, f"Disk check failed: {disk_message}"
        print(f"✓ {disk_message}")
        
        # Check swap
        swap_ok, swap_message, swap_info = self.swap_check.check_swap()
        if not swap_ok:
            return False, f"Swap check failed: {swap_message}"
        print(f"✓ {swap_message}")
        
        # Check clock (warn only)
        clock_ok, clock_message, clock_info = self.clock_check.check_sync()
        print(f"✓ {clock_message}")
        
        return True, "All prerequisites validated"
    
    def _display_eula(self) -> bool:
        """
        Display EULA and get acceptance.
        
        Returns:
            True if accepted, False otherwise
        """
        if not self.EULA_PATH.exists():
            print("✗ EULA file not found. Installation cannot proceed.", file=sys.stderr)
            return False
        
        # Read EULA
        try:
            with open(self.EULA_PATH, 'r') as f:
                eula_content = f.read()
        except Exception as e:
            print(f"✗ Error reading EULA: {e}", file=sys.stderr)
            return False
        
        # Display EULA
        print("\n" + "="*80)
        print("END USER LICENSE AGREEMENT (EULA)")
        print("="*80)
        if eula_content.strip():
            print(eula_content)
        else:
            print("EULA content will be provided here.")
        print("="*80 + "\n")
        
        # Get acceptance
        while True:
            response = input("Do you accept the EULA? (yes/no): ").strip().lower()
            if response in ['yes', 'y']:
                return True
            elif response in ['no', 'n']:
                print("EULA not accepted. Installation aborted.", file=sys.stderr)
                return False
            else:
                print("Please enter 'yes' or 'no'")
    
    def _configure_retention(self) -> bool:
        """
        Configure retention settings.
        Applies defaults if user skips.
        
        Returns:
            True if configured successfully
        """
        print("\n" + "="*80)
        print("RETENTION CONFIGURATION")
        print("="*80)
        print("Configure data retention policies.")
        print("Press Enter to use defaults, or provide custom values.\n")
        
        try:
            # Telemetry retention
            telemetry_input = input(f"Telemetry retention (months) [default: 6]: ").strip()
            telemetry_months = int(telemetry_input) if telemetry_input else None
            
            # Forensic retention
            forensic_input = input(f"Forensic retention (days) [default: 10]: ").strip()
            forensic_days = int(forensic_input) if forensic_input else None
            
            # Disk max usage
            disk_input = input(f"Disk max usage percent [default: 80]: ").strip()
            disk_percent = int(disk_input) if disk_input else None
            
            # Write retention config
            self.retention_writer.write_retention(
                telemetry_months=telemetry_months,
                forensic_days=forensic_days,
                disk_max_percent=disk_percent
            )
            
            # Validate
            is_valid, message = self.retention_validator.validate()
            if not is_valid:
                print(f"✗ Retention validation failed: {message}", file=sys.stderr)
                return False
            
            print(f"✓ {message}")
            config = self.retention_validator.get_config()
            print(f"  TELEMETRY_RETENTION_MONTHS: {config.get('TELEMETRY_RETENTION_MONTHS')}")
            print(f"  FORENSIC_RETENTION_DAYS: {config.get('FORENSIC_RETENTION_DAYS')}")
            print(f"  DISK_MAX_USAGE_PERCENT: {config.get('DISK_MAX_USAGE_PERCENT')}%")
            
            return True
        except ValueError as e:
            print(f"✗ Invalid retention value: {e}", file=sys.stderr)
            return False
        except Exception as e:
            print(f"✗ Error configuring retention: {e}", file=sys.stderr)
            return False
    
    def _generate_identity(self) -> bool:
        """
        Generate cryptographic identity.
        
        Returns:
            True if generated successfully
        """
        try:
            if self.identity_generator.identity_exists():
                print("✓ Identity already exists")
                print(f"  Identity Hash: {self.identity_generator.get_identity_hash()}")
            else:
                metadata = self.identity_generator.generate_identity()
                print("✓ Identity generated")
                print(f"  Identity Hash: {metadata['identity_hash']}")
            return True
        except Exception as e:
            print(f"✗ Error generating identity: {e}", file=sys.stderr)
            return False
    
    def _create_systemd_units(self) -> bool:
        """
        Create systemd unit files.
        
        Returns:
            True if created successfully
        """
        try:
            written = self.systemd_writer.write_service_units()
            print(f"✓ Generated {len(written)} systemd service units")
            print("  Note: Services are disabled by default and will not auto-start")
            return True
        except Exception as e:
            print(f"✗ Error creating systemd units: {e}", file=sys.stderr)
            return False
    
    def install(self) -> bool:
        """
        Run complete installation process.
        
        Returns:
            True if installation successful
        """
        print("="*80)
        print("RANSOMEYE INSTALLER")
        print("="*80)
        print(f"Version: {self.VERSION}\n")
        
        # Step 1: Validate prerequisites
        print("[1/7] Validating prerequisites...")
        is_valid, message = self._validate_prerequisites()
        if not is_valid:
            print(f"✗ {message}", file=sys.stderr)
            return False
        
        # Step 2: Display and accept EULA
        print("\n[2/7] EULA acceptance...")
        if not self._display_eula():
            return False
        print("✓ EULA accepted")
        
        # Step 3: Configure retention
        print("\n[3/7] Configuring retention...")
        if not self._configure_retention():
            return False
        
        # Step 4: Generate identity
        print("\n[4/7] Generating cryptographic identity...")
        if not self._generate_identity():
            return False
        
        # Step 5: Create systemd units
        print("\n[5/8] Creating systemd units...")
        if not self._create_systemd_units():
            return False
        
        # Step 6: Generate install manifest
        print("\n[6/8] Generating install manifest...")
        try:
            manifest_path = self.manifest_generator.write_manifest()
            print(f"✓ Install manifest generated: {manifest_path}")
            print(f"  Modules installed: {len(self.manifest_generator.generate_manifest()['modules'])}")
        except Exception as e:
            print(f"✗ Error generating manifest: {e}", file=sys.stderr)
            return False
        
        # Step 7: Save install state
        print("\n[7/8] Saving installation state...")
        try:
            state = self.state_manager.create_state(
                version=self.VERSION,
                eula_accepted=True,
                retention_configured=True,
                identity_generated=True,
                state='INSTALLED'
            )
            self.state_manager.save_state(state)
            print("✓ Installation state saved and signed")
        except Exception as e:
            print(f"✗ Error saving state: {e}", file=sys.stderr)
            return False
        
        # Step 8: Summary
        print("\n[8/8] Installation complete!")
        print("\n" + "="*80)
        print("INSTALLATION SUMMARY")
        print("="*80)
        print("✓ Prerequisites validated")
        print("✓ EULA accepted")
        print("✓ Retention configured")
        print("✓ Cryptographic identity generated")
        print("✓ Systemd units created (disabled by default)")
        print("✓ Install manifest generated")
        print("✓ Installation state saved")
        
        # Display installed modules
        service_modules = self.module_resolver.get_service_modules()
        standalone_modules = self.module_resolver.get_standalone_modules()
        
        print(f"\nInstalled modules:")
        print(f"  Service modules: {len(service_modules)}")
        for module in service_modules:
            print(f"    ✓ {module}")
        
        if standalone_modules:
            print(f"\n  Standalone agents (not installed by main installer): {len(standalone_modules)}")
            for module in standalone_modules:
                print(f"    ⚠ {module} (use dedicated installer)")
        
        print("\nNext steps:")
        print("  1. Review systemd units in /home/ransomeye/rebuild/systemd/")
        print("  2. Install units: sudo cp systemd/*.service /etc/systemd/system/")
        print("  3. Reload systemd: sudo systemctl daemon-reload")
        print("  4. Enable services: sudo systemctl enable ransomeye-*")
        print("  5. Start services: sudo systemctl start ransomeye-*")
        print("="*80)
        
        return True


def main():
    """CLI entry point for installer."""
    installer = RansomEyeInstaller()
    
    try:
        success = installer.install()
        sys.exit(0 if success else 1)
    except KeyboardInterrupt:
        print("\n\nInstallation cancelled by user.", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"\n✗ Fatal error during installation: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == '__main__':
    main()

