# Path and File Name : /home/ransomeye/rebuild/ransomeye_installer/manifest_generator.py
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Generates install manifest with installed modules, hashes, and metadata

"""
Install Manifest Generator: Creates verifiable install manifest at install time.
"""

import json
import hashlib
import sys
from pathlib import Path
from datetime import datetime
from typing import Dict, Optional

try:
    from .module_resolver import ModuleResolver
except ImportError:
    ModuleResolver = None


class ManifestGenerator:
    """Generates and manages install manifest."""
    
    MANIFEST_PATH = Path("/var/lib/ransomeye/install_manifest.json")
    PROJECT_ROOT = Path("/home/ransomeye/rebuild")
    GUARDRAILS_SPEC = PROJECT_ROOT / "core/guardrails/guardrails.yaml"
    
    def __init__(self):
        self.manifest_path = self.MANIFEST_PATH
        self.resolver = ModuleResolver() if ModuleResolver else None
    
    def generate_manifest(self) -> Dict:
        """
        Generate install manifest with all installed modules.
        
        Returns:
            Dictionary with manifest data
        """
        manifest = {
            'install_timestamp': datetime.utcnow().isoformat() + 'Z',
            'project_root': str(self.PROJECT_ROOT),
            'installer_version': '1.0.0',
            'modules': {},
            'systemd_units': [],
            'guardrails_spec_hash': self._get_guardrails_hash(),
        }
        
        if self.resolver:
            # Use resolver to get all modules
            all_modules = self.resolver.get_all_modules()
            
            for module_name, module_info in all_modules.items():
                module_path = Path(module_info['path'])
                module_type = module_info['type']
                
                # Compute module hash
                module_hash = self._compute_module_hash(module_path)
                
                # Get phase number if available
                phase_number = self._get_phase_number(module_name)
                
                manifest['modules'][module_name] = {
                    'path': str(module_path),
                    'type': module_type,
                    'hash': module_hash,
                    'phase': phase_number,
                }
                
                # Add systemd unit if it's a service module
                if module_type == 'service':
                    service_name = module_name.replace('ransomeye_', 'ransomeye-')
                    systemd_unit = f"{service_name}.service"
                    manifest['systemd_units'].append({
                        'name': systemd_unit,
                        'module': module_name,
                        'path': f"/home/ransomeye/rebuild/systemd/{systemd_unit}",
                    })
        else:
            # Fallback: minimal manifest
            manifest['modules'] = {}
            manifest['warning'] = 'Module resolver not available - manifest incomplete'
        
        return manifest
    
    def _compute_module_hash(self, module_path: Path) -> str:
        """
        Compute hash of module directory.
        
        Args:
            module_path: Path to module directory
            
        Returns:
            SHA-256 hash (first 16 chars)
        """
        try:
            # Get all files in module (recursive, but limit depth for performance)
            files = []
            for item in module_path.rglob('*'):
                if item.is_file():
                    # Skip large files and build artifacts
                    if any(skip in str(item) for skip in ['target/', '__pycache__/', '.git/', 'node_modules/']):
                        continue
                    try:
                        # Get file size and mtime for hash
                        stat = item.stat()
                        files.append(f"{item.relative_to(module_path)}:{stat.st_size}:{stat.st_mtime}")
                    except Exception:
                        continue
            
            # Create hash from file list
            content = '\n'.join(sorted(files))
            return hashlib.sha256(content.encode()).hexdigest()[:16]
        except Exception as e:
            return f"error:{hash(str(e)) % 10000}"
    
    def _get_phase_number(self, module_name: str) -> Optional[int]:
        """Get phase number for module from MODULE_PHASE_MAP.yaml."""
        try:
            map_path = self.PROJECT_ROOT / "MODULE_PHASE_MAP.yaml"
            if not map_path.exists():
                return None
            
            import yaml
            with open(map_path, 'r') as f:
                module_map = yaml.safe_load(f)
            
            if not module_map or 'modules' not in module_map:
                return None
            
            for module in module_map['modules']:
                if module.get('module_name') == module_name:
                    return module.get('phase_number')
        except Exception:
            pass
        
        return None
    
    def _get_guardrails_hash(self) -> Optional[str]:
        """Get guardrails specification hash."""
        try:
            if not self.GUARDRAILS_SPEC.exists():
                return None
            
            import yaml
            with open(self.GUARDRAILS_SPEC, 'r') as f:
                guardrails = yaml.safe_load(f)
            
            return guardrails.get('spec_hash') if guardrails else None
        except Exception:
            return None
    
    def write_manifest(self, manifest: Optional[Dict] = None) -> Path:
        """
        Write manifest to disk.
        
        Args:
            manifest: Manifest dictionary (generates if None)
            
        Returns:
            Path to written manifest file
        """
        if manifest is None:
            manifest = self.generate_manifest()
        
        # Ensure directory exists
        self.manifest_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Write manifest
        with open(self.manifest_path, 'w') as f:
            json.dump(manifest, f, indent=2, sort_keys=True)
        
        # Set permissions
        os.chmod(self.manifest_path, 0o644)
        
        return self.manifest_path
    
    def load_manifest(self) -> Optional[Dict]:
        """
        Load existing manifest from disk.
        
        Returns:
            Manifest dictionary or None if not found
        """
        if not self.manifest_path.exists():
            return None
        
        try:
            with open(self.manifest_path, 'r') as f:
                return json.load(f)
        except Exception:
            return None


def main():
    """CLI entry point for manifest generator."""
    generator = ManifestGenerator()
    manifest = generator.generate_manifest()
    manifest_path = generator.write_manifest(manifest)
    
    print(f"✓ Install manifest generated: {manifest_path}")
    print(f"  Modules: {len(manifest['modules'])}")
    print(f"  Systemd units: {len(manifest.get('systemd_units', []))}")


if __name__ == '__main__':
    main()

