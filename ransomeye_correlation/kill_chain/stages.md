# Kill-Chain Stages

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_correlation/kill_chain/stages.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Kill-chain stage definitions based on MITRE ATT&CK framework

## Overview

The correlation engine uses the MITRE ATT&CK kill-chain model to track attack progression. Stages are deterministic and never regress.

## Stages

### 1. Initial
- Starting state for all entities
- No attack activity detected

### 2. Reconnaissance
- Information gathering activities
- Port scanning, network scanning, DNS enumeration

### 3. Weaponization
- Malware creation and preparation
- Payload development, exploit preparation

### 4. Delivery
- Attack vector delivery
- Email attachments, web downloads, USB insertion

### 5. Exploitation
- Vulnerability exploitation
- Code injection, buffer overflow, exploit execution

### 6. Installation
- Persistence mechanisms
- Backdoor installation, service creation, scheduled tasks

### 7. Command & Control
- C2 communication establishment
- Beacon communication, command channels, data exfiltration

### 8. Actions on Objectives
- Final attack objectives
- Ransomware execution, data encryption, file deletion

### 9. Alerted
- Terminal state after alert generation
- No further state transitions

## State Transitions

States can only transition forward in the kill-chain. Regression is not allowed and indicates state corruption.

Valid transitions:
- Initial → Reconnaissance
- Reconnaissance → Weaponization
- Weaponization → Delivery
- Delivery → Exploitation
- Exploitation → Installation
- Installation → Command & Control
- Command & Control → Actions on Objectives
- Actions on Objectives → Alerted

## Determinism

- Same events → same stage progression
- No probabilistic stage assignment
- Explicit transitions only
- State corruption → ENGINE HALT

