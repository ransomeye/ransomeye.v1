# RansomEye DPI Probe

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_dpi_probe/README.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Stand-alone DPI Probe sensor for high-throughput network packet inspection

## Overview

The DPI Probe is a **stand-alone, untrusted sensor** that performs passive network inspection. It never enforces policy, never runs AI, and never stores long-term state.

## Responsibilities

- Passive network inspection
- High-throughput packet processing (10Gbps+)
- Feature extraction only
- No decisions, no blocking

## Hard Rules

- Zero packet modification
- Zero enforcement
- Zero AI
- Bounded buffers
- Backpressure support

## Components

- **Capture Engine**: High-throughput packet capture using libpcap (passive only)
- **Flow Assembler**: Tracks network flows and assembles packets
- **Feature Extractor**: Extracts flow features (NO AI classification, metadata only)
- **Event Signer**: Signs all telemetry with RSA-4096-PSS-SHA256
- **Backpressure Handler**: Manages bounded buffers and backpressure signals
- **Disk Buffer**: Persistent buffering when Core unavailable (fail-open)
- **Output Formatter**: Telemetry serialization and validation
- **Health Reporter**: Reports sensor health status
- **Transport Client**: mTLS client for sending signed events to Core
- **Security Modules**: Identity, signature verification, replay protection, trust chain

## Configuration

All configuration via environment variables:

- `CORE_API_URL`: Core API endpoint (default: `https://localhost:8443`)
- `CAPTURE_IFACE`: Network interface to capture (default: `eth0`)
- `BUFFER_DIR`: Buffer directory (default: `/var/lib/ransomeye/dpi_probe/buffer`)
- `PROBE_CERT_PATH`: Client certificate path
- `PROBE_KEY_PATH`: Client private key path
- `CA_CERT_PATH`: CA certificate path
- `MAX_BUFFER_SIZE_MB`: Maximum buffer size in MB (default: 1024)
- `BACKPRESSURE_THRESHOLD`: Backpressure threshold (default: 8192)
- `FLOW_TIMEOUT_SECONDS`: Flow timeout in seconds (default: 300)

## Communication

- mTLS authentication with per-instance certificates
- Event signing with RSA-4096-PSS-SHA256
- Replay protection via nonce
- Backpressure handling

## Testing

Run integration tests:

```bash
cd ransomeye_dpi_probe
cargo test
```

Tests cover:
- **Identity spoofing prevention**: Rejects spoofed probe identity
- **Event signing verification**: Validates all events are signed
- **Backpressure handling**: Correctly handles Core backpressure signals
- **Core unavailability tolerance**: Buffers to disk when Core unavailable
- **Resource exhaustion handling**: Graceful degradation under resource pressure

## Architecture

See detailed documentation in `docs/`:
- `dpi_architecture.md`: Overall architecture and design
- `performance_model.md`: Performance characteristics and scalability
- `backpressure_behavior.md`: Backpressure handling details
- `security_model.md`: Security architecture and threat model
- `failure_modes.md`: Failure handling and recovery

## Protocol

Event schema and compatibility documentation in `protocol/`:
- `dpi_event_schema.json`: JSON schema for telemetry events
- `compatibility.md`: Protocol versioning and compatibility

## Security

Security modules in `security/`:
- Identity management and verification
- Cryptographic signature verification
- Replay attack protection
- Trust chain validation

## Build

```bash
cargo build --release
```

## Run

```bash
./target/release/ransomeye_dpi_probe
```

