# Path and File Name : /home/ransomeye/rebuild/install_training_dependencies.sh
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Installs Python dependencies for AI/ML training modules

#!/bin/bash

set -e

echo "Installing Python dependencies for RansomEye AI/ML training..."

# Install system packages (preferred for externally-managed environments)
if command -v apt-get &> /dev/null; then
    echo "Installing via apt-get..."
    sudo apt-get update -qq
    sudo apt-get install -y \
        python3-sklearn \
        python3-numpy \
        python3-scipy \
        python3-requests \
        python3-pip
    echo "✓ System packages installed"
elif command -v yum &> /dev/null; then
    echo "Installing via yum..."
    sudo yum install -y \
        python3-scikit-learn \
        python3-numpy \
        python3-scipy \
        python3-requests \
        python3-pip
    echo "✓ System packages installed"
else
    echo "ERROR: Unsupported package manager"
    exit 1
fi

# Verify installation
echo "Verifying installation..."
python3 -c "
import sklearn
import numpy
import scipy
import requests
print('✓ All dependencies installed successfully')
print(f'  sklearn: {sklearn.__version__}')
print(f'  numpy: {numpy.__version__}')
print(f'  scipy: {scipy.__version__}')
print(f'  requests: {requests.__version__}')
"

echo ""
echo "✓ Training dependencies installation complete"

