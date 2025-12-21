# Path and File Name : /home/ransomeye/rebuild/ransomeye_posture_engine/__main__.py
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Main entry point for posture engine daemon

"""
Main entry point for RansomEye Posture Engine daemon.
"""

import asyncio
import sys
import os
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent))

from ransomeye_posture_engine.engine.posture_daemon import PostureDaemon
from ransomeye_posture_engine.config import Config
from ransomeye_posture_engine.logging_config import setup_logging

async def main():
    """Main entry point."""
    setup_logging()
    
    config = Config.from_env()
    daemon = PostureDaemon(config)
    
    try:
        await daemon.start()
    except KeyboardInterrupt:
        print("\nShutting down...")
    finally:
        await daemon.stop()

if __name__ == "__main__":
    asyncio.run(main())

