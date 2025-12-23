# Path and File Name : /home/ransomeye/rebuild/ransomeye_installer/services/systemd_writer.py
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Writes all systemd unit files to unified systemd directory - all units disabled by default

"""
Systemd Writer: Writes all systemd unit files.
All units are disabled by default and depend on installer state.
"""

import os
import sys
from pathlib import Path
from typing import List, Dict

# Import module resolver for canonical module enumeration
try:
    from ..module_resolver import ModuleResolver
except ImportError:
    # Fallback if import fails
    ModuleResolver = None


class SystemdWriter:
    """Writes systemd unit files."""
    
    SYSTEMD_DIR = Path("/home/ransomeye/rebuild/systemd")
    INSTALL_STATE_FILE = Path("/home/ransomeye/rebuild/ransomeye_installer/config/install_state.json")
    
    def __init__(self):
        self.SYSTEMD_DIR.mkdir(parents=True, exist_ok=True)
        
        # Use module resolver to get canonical service modules
        if ModuleResolver:
            self.resolver = ModuleResolver()
            # Get service modules that actually exist on disk
            self.CORE_MODULES = self.resolver.get_service_modules()
        else:
            # Fallback: hardcoded list (will be validated)
            self.CORE_MODULES = [
                'ransomeye_ai_advisory',
                'ransomeye_correlation',
                'ransomeye_enforcement',
                'ransomeye_ingestion',
                'ransomeye_intelligence',
                'ransomeye_policy',
                'ransomeye_reporting',
            ]
            self.resolver = None
        
        # CRITICAL: Validate all modules exist on disk before proceeding
        self._validate_modules_exist()
    
    def _validate_modules_exist(self) -> None:
        """
        Validate that all modules in CORE_MODULES exist on disk.
        FAIL-CLOSED: Raises SystemExit if any module directory is missing.
        """
        project_root = Path("/home/ransomeye/rebuild")
        missing_modules = []
        
        for module_name in self.CORE_MODULES:
            if self.resolver:
                # Use resolver to validate
                if not self.resolver.validate_module_exists(module_name):
                    missing_modules.append(module_name)
            else:
                # Fallback validation
                module_dir = project_root / module_name
                if not module_dir.exists() or not module_dir.is_dir():
                    missing_modules.append(module_name)
        
        if missing_modules:
            error_msg = (
                f"BUILD FAILURE: Referenced modules do not exist on disk:\n"
                f"  Missing: {', '.join(missing_modules)}\n"
                f"  All module references MUST point to existing directories.\n"
                f"  Check MODULE_PHASE_MAP.yaml for canonical module mappings."
            )
            print(f"ERROR: {error_msg}", file=sys.stderr)
            sys.exit(1)
    
    def _generate_service_unit(self, module_name: str) -> str:
        """
        Generate systemd service unit content.
        
        Args:
            module_name: Module name (e.g., 'ransomeye_ai_core')
        
        Returns:
            Service unit content
        """
        service_name = module_name.replace('ransomeye_', 'ransomeye-')
        
        # Generate service-specific directory names
        state_dir_name = service_name.replace('ransomeye-', '')
        
        unit_content = f"""# Path and File Name : /home/ransomeye/rebuild/systemd/{service_name}.service
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Systemd service unit for {module_name}
# CRITICAL: Rootless runtime enforcement - MUST NOT run as root (UID 0)

[Unit]
Description=RansomEye {module_name}
After=network.target
Requires=network.target
ConditionPathExists={self.INSTALL_STATE_FILE}

[Service]
Type=simple
User=ransomeye
Group=ransomeye
WorkingDirectory=/home/ransomeye/rebuild
RuntimeDirectory=ransomeye/{state_dir_name}
StateDirectory=ransomeye/{state_dir_name}
ExecStart=/usr/bin/python3 -m {module_name}
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

# Security hardening - Rootless runtime enforcement
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/home/ransomeye/rebuild /var/lib/ransomeye/{state_dir_name} /run/ransomeye/{state_dir_name}

# Capability-based privileges (no root required)
CapabilityBoundingSet=CAP_NET_BIND_SERVICE CAP_NET_RAW CAP_SYS_NICE
AmbientCapabilities=
PrivateUsers=false

[Install]
WantedBy=multi-user.target
"""
        return unit_content
    
    def write_service_units(self) -> List[Path]:
        """
        Write all service unit files.
        
        Returns:
            List of written unit file paths
        
        Raises:
            SystemExit: If any module directory does not exist (fail-closed)
        """
        written_files = []
        
        # Re-validate before writing (defense in depth)
        self._validate_modules_exist()
        
        for module in self.CORE_MODULES:
            service_name = module.replace('ransomeye_', 'ransomeye-')
            unit_file = self.SYSTEMD_DIR / f"{service_name}.service"
            
            content = self._generate_service_unit(module)
            
            with open(unit_file, 'w') as f:
                f.write(content)
            
            os.chmod(unit_file, 0o644)
            written_files.append(unit_file)
        
        return written_files
    
    def install_units(self) -> bool:
        """
        Install systemd units (copy to /etc/systemd/system).
        Does NOT enable them.
        
        Returns:
            True if successful
        """
        try:
            import shutil
            
            for unit_file in self.SYSTEMD_DIR.glob("*.service"):
                target = Path(f"/etc/systemd/system/{unit_file.name}")
                shutil.copy2(unit_file, target)
                os.chmod(target, 0o644)
            
            # Reload systemd
            import subprocess
            subprocess.run(['systemctl', 'daemon-reload'], check=True, timeout=30)
            
            return True
        except Exception:
            return False


def main():
    """CLI entry point for systemd writer."""
    writer = SystemdWriter()
    
    written = writer.write_service_units()
    print(f"✓ Generated {len(written)} systemd service units")
    
    for unit_file in written:
        print(f"  {unit_file}")


if __name__ == '__main__':
    main()

