# Path and File Name : /home/ransomeye/rebuild/ransomeye_installer/retention/retention_writer.py
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Writes retention.txt configuration with validation and defaults

"""
Retention Writer: Writes retention.txt configuration.
Applies defaults if values not provided and validates all inputs.
"""

import os
from pathlib import Path
from typing import Optional, Dict, Tuple


class RetentionWriter:
    """Writes retention configuration to file."""
    
    DEFAULT_VALUES = {
        'TELEMETRY_RETENTION_MONTHS': 6,
        'FORENSIC_RETENTION_DAYS': 10,
        'DISK_MAX_USAGE_PERCENT': 80
    }
    
    MIN_VALUES = {
        'TELEMETRY_RETENTION_MONTHS': 1,
        'FORENSIC_RETENTION_DAYS': 1,
        'DISK_MAX_USAGE_PERCENT': 50
    }
    
    MAX_VALUES = {
        'TELEMETRY_RETENTION_MONTHS': 84,  # 7 years
        'FORENSIC_RETENTION_DAYS': 3650,  # 10 years
        'DISK_MAX_USAGE_PERCENT': 100
    }
    
    def __init__(self, config_path: str = "/home/ransomeye/rebuild/config/retention.txt"):
        self.config_path = Path(config_path)
        self.config_path.parent.mkdir(parents=True, exist_ok=True)
    
    def validate_value(self, key: str, value: int) -> tuple[bool, str]:
        """
        Validate a retention value.
        
        Args:
            key: Configuration key
            value: Value to validate
        
        Returns:
            Tuple of (is_valid: bool, error_message: str)
        """
        if key not in self.DEFAULT_VALUES:
            return False, f"Unknown configuration key: {key}"
        
        min_val = self.MIN_VALUES.get(key, 0)
        max_val = self.MAX_VALUES.get(key, 0)
        
        if value < min_val:
            return False, f"{key} must be >= {min_val}, got {value}"
        
        if value > max_val:
            return False, f"{key} must be <= {max_val}, got {value}"
        
        return True, ""
    
    def write_retention(self, telemetry_months: Optional[int] = None,
                       forensic_days: Optional[int] = None,
                       disk_max_percent: Optional[int] = None) -> Path:
        """
        Write retention configuration to file.
        
        Args:
            telemetry_months: Telemetry retention in months (default: 6)
            forensic_days: Forensic retention in days (default: 10)
            disk_max_percent: Disk max usage percent (default: 80)
        
        Returns:
            Path to written configuration file
        """
        # Apply defaults
        config = {
            'TELEMETRY_RETENTION_MONTHS': telemetry_months if telemetry_months is not None else self.DEFAULT_VALUES['TELEMETRY_RETENTION_MONTHS'],
            'FORENSIC_RETENTION_DAYS': forensic_days if forensic_days is not None else self.DEFAULT_VALUES['FORENSIC_RETENTION_DAYS'],
            'DISK_MAX_USAGE_PERCENT': disk_max_percent if disk_max_percent is not None else self.DEFAULT_VALUES['DISK_MAX_USAGE_PERCENT']
        }
        
        # Validate all values
        for key, value in config.items():
            is_valid, error = self.validate_value(key, value)
            if not is_valid:
                raise ValueError(f"Invalid retention configuration: {error}")
        
        # Write configuration
        with open(self.config_path, 'w') as f:
            f.write("# Path and File Name : /home/ransomeye/rebuild/config/retention.txt\n")
            f.write("# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU\n")
            f.write("# Details of functionality of this file: Data retention policy configuration - defines retention periods for telemetry, forensics, and disk usage thresholds\n\n")
            f.write("# RansomEye Data Retention Policy Configuration\n")
            f.write("# All values are enforced at runtime and validated in CI/CD\n\n")
            f.write(f"TELEMETRY_RETENTION_MONTHS={config['TELEMETRY_RETENTION_MONTHS']}\n")
            f.write(f"FORENSIC_RETENTION_DAYS={config['FORENSIC_RETENTION_DAYS']}\n")
            f.write(f"DISK_MAX_USAGE_PERCENT={config['DISK_MAX_USAGE_PERCENT']}\n\n")
            f.write("# Note: AI Training Artifacts have a mandatory minimum retention of 2 years\n")
            f.write("# AI artifacts cannot be deleted by disk pressure - only via explicit operator approval\n")
        
        return self.config_path
    
    def write_defaults(self) -> Path:
        """
        Write default retention configuration.
        
        Returns:
            Path to written configuration file
        """
        return self.write_retention()


def main():
    """CLI entry point for retention writer."""
    import argparse
    
    parser = argparse.ArgumentParser(description='RansomEye Retention Writer')
    parser.add_argument('--telemetry-months', type=int, default=None,
                       help='Telemetry retention in months (default: 6)')
    parser.add_argument('--forensic-days', type=int, default=None,
                       help='Forensic retention in days (default: 10)')
    parser.add_argument('--disk-max-percent', type=int, default=None,
                       help='Disk max usage percent (default: 80)')
    parser.add_argument('--defaults', action='store_true',
                       help='Write default values')
    
    args = parser.parse_args()
    
    writer = RetentionWriter()
    
    if args.defaults:
        path = writer.write_defaults()
    else:
        path = writer.write_retention(
            telemetry_months=args.telemetry_months,
            forensic_days=args.forensic_days,
            disk_max_percent=args.disk_max_percent
        )
    
    print(f"✓ Retention configuration written to: {path}")


if __name__ == '__main__':
    main()

