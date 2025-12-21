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


class SystemdWriter:
    """Writes systemd unit files."""
    
    SYSTEMD_DIR = Path("/home/ransomeye/rebuild/systemd")
    INSTALL_STATE_FILE = Path("/home/ransomeye/rebuild/ransomeye_installer/config/install_state.json")
    
    # Core modules that require systemd units
    # CRITICAL: Only modules that EXIST on disk are listed here
    # All phantom modules have been removed per MODULE_PHASE_MAP.yaml
    # Module mappings:
    #   - ransomeye_ai_core -> ransomeye_ai_advisory (Phase 8)
    #   - ransomeye_alert_engine -> ransomeye_intelligence + ransomeye_policy
    #   - ransomeye_killchain_core -> ransomeye_correlation (Phase 5)
    #   - ransomeye_threat_correlation -> ransomeye_correlation (Phase 5)
    #   - ransomeye_threat_intel_engine -> ransomeye_intelligence (Phase 3)
    #   - ransomeye_forensic -> ransomeye_reporting (Phase 10)
    #   - ransomeye_response -> ransomeye_enforcement (Phase 7)
    #   - ransomeye_db_core -> library-based (no service)
    #   - ransomeye_hnmp_engine -> NOT_FOUND (Phase 19, needs creation)
    #   - ransomeye_incident_summarizer -> NOT_FOUND (Phase 5, needs creation)
    #   - ransomeye_llm -> NOT_FOUND (Phase 5, needs creation)
    #   - ransomeye_master_core -> ransomeye_operations (Phase 1, tool, no service)
    #   - ransomeye_net_scanner -> NOT_FOUND (Phase 9, needs creation)
    #   - ransomeye_ui -> NOT_FOUND (Phase 11, needs creation)
    CORE_MODULES = [
        'ransomeye_ai_advisory',      # Phase 8: AI Advisory (covers AI Core functionality)
        'ransomeye_correlation',      # Phase 5: Correlation Engine (covers killchain + threat correlation)
        'ransomeye_enforcement',      # Phase 7: Enforcement (covers response functionality)
        'ransomeye_ingestion',        # Phase 4: Event Ingestion
        'ransomeye_intelligence',     # Phase 3: Intelligence (covers alert engine + threat intel)
        'ransomeye_policy',            # Phase 6: Policy Engine (covers alert engine functionality)
        'ransomeye_reporting',        # Phase 10: Reporting (covers forensic functionality)
    ]
    
    def __init__(self):
        self.SYSTEMD_DIR.mkdir(parents=True, exist_ok=True)
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

