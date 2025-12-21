# RansomEye Linux Agent

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_linux_agent/README.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Stand-alone Linux Agent sensor for host telemetry collection

## Overview

The Linux Agent is a **stand-alone, untrusted sensor** that collects host telemetry. It never enforces policy, never runs AI, and never stores long-term state.

## Responsibilities

- Process telemetry
- File activity telemetry
- Auth activity telemetry
- Controlled response hooks (NOT enforcement)

## Hard Rules

- No kernel patching
- No persistent daemons beyond agent
- No swap dependency
- Minimal privileges
- Signed telemetry only

## Components

- **Telemetry Collector**: Collects process, file, and auth events
- **Event Signer**: Signs all telemetry with RSA-4096-PSS-SHA256
- **Transport Client**: mTLS client for sending signed events to Core
- **Backpressure Handler**: Manages bounded buffers and backpressure signals
- **Health Reporter**: Reports sensor health status

## Configuration

All configuration via environment variables:

- `CORE_API_URL`: Core API endpoint (default: `https://localhost:8443`)
- `BUFFER_DIR`: Buffer directory (default: `/var/lib/ransomeye/linux_agent/buffer`)
- `AGENT_CERT_PATH`: Client certificate path
- `AGENT_KEY_PATH`: Client private key path
- `CA_CERT_PATH`: CA certificate path
- `MAX_BUFFER_SIZE_MB`: Maximum buffer size in MB (default: 512)
- `BACKPRESSURE_THRESHOLD`: Backpressure threshold (default: 4096)
- `TELEMETRY_INTERVAL_SECONDS`: Telemetry collection interval (default: 1)
- `MONITOR_PATHS`: Comma-separated paths to monitor (default: `/,/home,/var,/tmp`)

## Communication

- mTLS authentication with per-instance certificates
- Event signing with RSA-4096-PSS-SHA256
- Replay protection via nonce
- Backpressure handling

## Testing

Run integration tests:

```bash
cd ransomeye_linux_agent
cargo test
```

Tests cover:
- Identity spoofing prevention
- Event signing verification
- Backpressure handling
- Core unavailability tolerance
- Resource exhaustion handling

## Build

```bash
cargo build --release
```

## Run

```bash
./target/release/ransomeye_linux_agent
```

