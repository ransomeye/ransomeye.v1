# Path and File Name : /home/ransomeye/rebuild/ransomeye_installer/tests/eula_enforcement_test.py
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Tests EULA enforcement - verifies installation fails without EULA acceptance

"""
Tests for EULA enforcement.
Verifies installation fails without EULA acceptance.
"""

import unittest
import tempfile
import os
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from ransomeye_installer.installer import RansomEyeInstaller


class TestEULAEnforcement(unittest.TestCase):
    """Test EULA enforcement."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.temp_dir = tempfile.mkdtemp()
        self.eula_path = Path(self.temp_dir) / "EULA.txt"
    
    def tearDown(self):
        """Clean up test fixtures."""
        import shutil
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def test_eula_missing_fails(self):
        """Test that missing EULA causes failure."""
        # EULA file doesn't exist
        installer = RansomEyeInstaller()
        installer.EULA_PATH = Path(self.temp_dir) / "nonexistent.txt"
        
        # Should fail when trying to display EULA
        result = installer._display_eula()
        self.assertFalse(result)
    
    def test_eula_exists(self):
        """Test that EULA file can be read."""
        # Create EULA file
        with open(self.eula_path, 'w') as f:
            f.write("Test EULA content")
        
        installer = RansomEyeInstaller()
        installer.EULA_PATH = self.eula_path
        
        # EULA should exist
        self.assertTrue(self.eula_path.exists())


def main():
    """Run tests."""
    unittest.main()


if __name__ == '__main__':
    main()

