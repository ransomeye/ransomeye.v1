# RansomEye — Enterprise Security Platform

**RansomEye** is a comprehensive, enterprise-grade cybersecurity platform designed to detect, analyze, and respond to ransomware and advanced persistent threats (APTs). Built with 23 integrated phases, RansomEye provides end-to-end security orchestration, automated response, and AI-powered threat intelligence.

## 🏗️ Architecture

RansomEye is organized into 23 distinct phases, each serving a critical security function:

### Core Modules
- **Phase 1**: Core Engine & Installer
- **Phase 2**: AI Core & Model Registry
- **Phase 3**: Alert Engine & Policy Manager
- **Phase 4**: KillChain & Forensic Dump
- **Phase 5**: LLM Summarizer
- **Phase 6**: Incident Response & Playbooks
- **Phase 7**: SOC Copilot
- **Phase 8**: Threat Correlation Engine
- **Phase 9**: Network Scanner
- **Phase 10**: DB Core (PostgreSQL)
- **Phase 11**: UI & Dashboards
- **Phase 12**: Orchestrator (Master Flow)

### Advanced Modules
- **Phase 13**: Forensic Engine (Advanced)
- **Phase 14**: LLM Behavior Summarizer (Expanded)
- **Phase 15**: SOC Copilot (Advanced)
- **Phase 16**: Deception Framework
- **Phase 17**: AI Assistant (Governor Mode)
- **Phase 18**: Threat Intelligence Feed Engine
- **Phase 19**: HNMP Engine
- **Phase 20**: Global Validator

### Standalone Agents
- **Phase 21**: Linux Agent
- **Phase 22**: Windows Agent
- **Phase 23**: DPI Probe

### Supporting Infrastructure
- **Guardrails**: Fail-closed security enforcement
- **Retention**: Data retention and disk management
- **Trust**: Cryptographic signing and verification

## 🚀 Quick Start

### Prerequisites
- Python 3.8+
- PostgreSQL 12+
- Linux (Ubuntu 20.04+ recommended)
- Systemd (for service management)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/ransomeye-rebuild.git
cd ransomeye-rebuild

# Run the installer
sudo ./install.sh

# Verify installation
./post_install_validator.py
```

### Configuration

1. Copy environment template:
```bash
cp .env.example .env
```

2. Configure database and API settings in `.env`

3. Start services:
```bash
sudo systemctl start ransomeye-master-core
sudo systemctl status ransomeye-*
```

## 📋 Features

- **AI-Powered Detection**: Machine learning models with SHAP explainability
- **Offline Operation**: Fully air-gapped capable, no internet dependencies
- **Real-time Monitoring**: Continuous threat detection and alerting
- **Automated Response**: Playbook-driven incident response
- **Forensic Analysis**: Complete kill chain reconstruction
- **Threat Intelligence**: Integrated feeds from MISP, OTX, Talos, ThreatFox
- **Compliance**: Built-in compliance scanning and reporting
- **Multi-Format Exports**: PDF, HTML, CSV reporting

## 🔒 Security

- **Fail-Closed Architecture**: System fails securely on violations
- **Cryptographic Signing**: All artifacts are signed and verified
- **Encrypted Storage**: Database encryption for sensitive data
- **No Hardcoded Secrets**: All configuration via environment variables
- **Rootless Operation**: Services run with minimal privileges

## 📊 Data Management

- **PostgreSQL Database**: Centralized data storage
- **7-Year Retention**: Configurable data retention policies
- **Automatic Cleanup**: Disk space management when >80% full
- **Partitioned Tables**: Optimized for large-scale data

## 🤖 AI & Machine Learning

- **Model Registry**: Centralized model management
- **SHAP Explainability**: All ML outputs include explainability
- **Offline Models**: Pre-trained models included (via Git LFS)
- **Adversarial Defense**: Built-in model robustness

## 📝 Reporting

All modules export reports in multiple formats:
- **PDF**: Formatted reports with branding
- **HTML**: Interactive web reports
- **CSV**: Machine-readable data exports
- **JSON**: Structured data (optional)

All reports include:
- Timestamp and build hash
- Model version information
- SHAP explainability context
- Footer: "© RansomEye.Tech | Support: Gagan@RansomEye.Tech"

## 🧪 Testing

```bash
# Run all tests
python -m pytest tests/

# Run specific module tests
python -m pytest ransomeye_guardrails/tests/

# Validate guardrails
python -m ransomeye_guardrails.scanner
```

## 📚 Documentation

- **Installation Guide**: See `install.sh` and `post_install_validator.py`
- **API Documentation**: See individual module READMEs
- **Architecture**: See project specification in user rules

## 🔧 Development

### Project Structure
```
/home/ransomeye/rebuild/
├── install.sh                 # Unified installer
├── uninstall.sh              # Unified uninstaller
├── requirements.txt          # Unified dependencies
├── systemd/                 # All systemd service files
├── logs/                    # Centralized logging
├── ransomeye_*/            # Individual phase modules
└── tests/                   # Test suites
```

### Code Standards
- All files must include mandatory header
- No hardcoded IPs, ports, or credentials
- Environment variables for all configuration
- SHAP explainability for all ML outputs
- Comprehensive test coverage

## 📞 Support

**Support Email**: Gagan@RansomEye.Tech

## 📄 License

Proprietary - RansomEye.Tech

## 🙏 Acknowledgments

Built with enterprise-excellent standards, designed to outperform leading cybersecurity solutions.

---

**© RansomEye.Tech | Support: Gagan@RansomEye.Tech**

