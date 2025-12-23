#!/bin/bash
# Path and File Name : /home/ransomeye/rebuild/TRAIN_COMMAND.sh
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Quick training command script

# Set environment variables
export MALWARBAZAAR_AUTH_KEY="483ce60ba7c8a3d7358e3c8afd6e6d23a746eb2a5a42479f"
export RANSOMWARE_LIVE_API_KEY="6c0cca08-3419-43e6-8014-0a4f87f353a3"

# Run training with threat intelligence feeds
python3 ransomeye_intelligence/baseline_pack/train_baseline_models.py --use-feeds

