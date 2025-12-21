# Path and File Name : /home/ransomeye/rebuild/ransomeye_installer/services/systemd_writer.py
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Writes all systemd unit files to unified systemd directory - all units disabled by default

"""
Systemd Writer: Writes all systemd unit files.
All units are disabled by default and depend on installer state.
"""

import os
from pathlib import Path
from typing import List, Dict


class SystemdWriter:
    """Writes systemd unit files."""
    
    SYSTEMD_DIR = Path("/home/ransomeye/rebuild/systemd")
    INSTALL_STATE_FILE = Path("/home/ransomeye/rebuild/ransomeye_installer/config/install_state.json")
    
    # Core modules that require systemd units
    CORE_MODULES = [
        'ransomeye_ai_core',
        'ransomeye_alert_engine',
        'ransomeye_db_core',
        'ransomeye_forensic',
        'ransomeye_hnmp_engine',
        'ransomeye_incident_summarizer',
        'ransomeye_killchain_core',
        'ransomeye_llm',
        'ransomeye_master_core',
        'ransomeye_net_scanner',
        'ransomeye_response',
        'ransomeye_threat_correlation',
        'ransomeye_threat_intel_engine',
        'ransomeye_ui',
    ]
    
    def __init__(self):
        self.SYSTEMD_DIR.mkdir(parents=True, exist_ok=True)
    
    def _generate_service_unit(self, module_name: str) -> str:
        """
        Generate systemd service unit content.
        
        Args:
            module_name: Module name (e.g., 'ransomeye_ai_core')
        
        Returns:
            Service unit content
        """
        service_name = module_name.replace('ransomeye_', 'ransomeye-')
        
        unit_content = f"""# Path and File Name : /home/ransomeye/rebuild/systemd/{service_name}.service
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Systemd service unit for {module_name}

[Unit]
Description=RansomEye {module_name}
After=network.target
Requires=network.target
ConditionPathExists={self.INSTALL_STATE_FILE}

[Service]
Type=simple
User=root
WorkingDirectory=/home/ransomeye/rebuild
ExecStart=/usr/bin/python3 -m {module_name}
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/home/ransomeye/rebuild

[Install]
WantedBy=multi-user.target
"""
        return unit_content
    
    def write_service_units(self) -> List[Path]:
        """
        Write all service unit files.
        
        Returns:
            List of written unit file paths
        """
        written_files = []
        
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

