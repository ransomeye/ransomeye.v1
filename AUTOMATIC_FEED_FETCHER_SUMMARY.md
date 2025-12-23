# Automatic Threat Intelligence Feed Fetcher

**Path and File Name:** `/home/ransomeye/rebuild/AUTOMATIC_FEED_FETCHER_SUMMARY.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Summary of automatic threat intelligence feed fetching system

## Overview

RansomEye now includes an **automatic threat intelligence feed fetcher** that:
- ✅ Runs daily at 2 AM
- ✅ Retries every hour if previous fetch failed (up to 24 retries)
- ✅ Checks for internet connectivity before attempting fetches
- ✅ Supports multiple threat intelligence sources
- ✅ Caches all feeds locally for offline training
- ✅ Automatically integrates with training pipelines

## Features

### Automatic Scheduling
- **Daily Run:** 2:00 AM (configurable)
- **Hourly Retries:** Every hour if previous fetch failed
- **Internet Detection:** Only attempts fetch when internet is available
- **Persistent State:** Tracks last successful fetch and retry count

### Supported Feed Sources

#### Primary Sources (Always Active)
1. **MalwareBazaar** - Malware samples
2. **Wiz.io** - Cloud threat landscape (STIX)
3. **Ransomware.live** - Ransomware groups and victims

#### Additional Sources (Auto-detected)
4. **URLhaus** (Abuse.ch) - Malicious URLs (no API key)
5. **ThreatFox** (Abuse.ch) - IOCs (no API key)
6. **AlienVault OTX** - Threat intelligence (requires `OTX_KEY`)
7. **VirusTotal** - Threat intelligence (requires `VIRUSTOTAL_KEY`)

### Retry Logic
- Maximum retries: 24 (once per hour for 24 hours)
- Retry interval: 1 hour
- Internet check before each attempt
- Graceful degradation (uses cached data if fetch fails)

## Installation

### Quick Install

```bash
# 1. Create directories
sudo mkdir -p /var/log/ransomeye /var/lib/ransomeye
sudo chown -R ransomeye:ransomeye /var/log/ransomeye /var/lib/ransomeye

# 2. Install systemd service and timer
sudo cp /home/ransomeye/rebuild/systemd/ransomeye-feed-fetcher.service /etc/systemd/system/
sudo cp /home/ransomeye/rebuild/systemd/ransomeye-feed-fetcher.timer /etc/systemd/system/

# 3. Enable and start
sudo systemctl daemon-reload
sudo systemctl enable ransomeye-feed-fetcher.timer
sudo systemctl start ransomeye-feed-fetcher.timer
```

### Verify Installation

```bash
# Check timer status
sudo systemctl status ransomeye-feed-fetcher.timer

# Check next run time
sudo systemctl list-timers ransomeye-feed-fetcher.timer

# View logs
sudo journalctl -u ransomeye-feed-fetcher.service -f
```

## Usage

### Manual Execution

```bash
# Run fetcher manually
sudo systemctl start ransomeye-feed-fetcher.service

# Or run directly
python3 /home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/ingestion/auto_feed_fetcher.py
```

### Adding Additional Sources

Edit `/etc/systemd/system/ransomeye-feed-fetcher.service`:

```ini
Environment="OTX_KEY=your-otx-api-key"
Environment="VIRUSTOTAL_KEY=your-virustotal-api-key"
```

Then reload:

```bash
sudo systemctl daemon-reload
sudo systemctl restart ransomeye-feed-fetcher.timer
```

## Monitoring

### Check Status

```bash
# Timer status
sudo systemctl status ransomeye-feed-fetcher.timer

# Last successful fetch
cat /var/lib/ransomeye/feed_fetcher_state.json

# Recent logs
sudo journalctl -u ransomeye-feed-fetcher.service --since "1 hour ago"
```

### Log Files

- Systemd logs: `sudo journalctl -u ransomeye-feed-fetcher.service`
- Application log: `/var/log/ransomeye/feed_fetcher.log`
- State file: `/var/lib/ransomeye/feed_fetcher_state.json`

## Integration with Training

The automatic fetcher integrates seamlessly with training:

```bash
# Training automatically uses latest cached feeds
python3 ransomeye_intelligence/baseline_pack/train_baseline_models.py --use-feeds
```

The training pipeline automatically:
1. Loads cached feeds from all sources
2. Enhances synthetic data with threat intelligence
3. Trains models with enriched data

## Configuration

### Timer Schedule

Edit `/etc/systemd/system/ransomeye-feed-fetcher.timer`:

```ini
# Daily at 2 AM
OnCalendar=*-*-* 02:00:00

# Hourly retries
OnCalendar=*-*-* *:00:00
```

### Environment Variables

Set in `/etc/systemd/system/ransomeye-feed-fetcher.service`:

```ini
Environment="MALWARBAZAAR_AUTH_KEY=..."
Environment="RANSOMWARE_LIVE_API_KEY=..."
Environment="OTX_KEY=..."  # Optional
Environment="VIRUSTOTAL_KEY=..."  # Optional
```

## Troubleshooting

### No Feeds Being Fetched

1. Check internet connectivity:
   ```bash
   ping -c 3 8.8.8.8
   ```

2. Check service logs:
   ```bash
   sudo journalctl -u ransomeye-feed-fetcher.service --since "1 hour ago"
   ```

3. Check state file:
   ```bash
   cat /var/lib/ransomeye/feed_fetcher_state.json
   ```

### Feed Fetch Failures

The fetcher will:
- Log errors to systemd journal
- Retry in 1 hour
- Use cached data if available
- Continue with other feeds if one fails

## Files Created

1. `ransomeye_intelligence/threat_intel/ingestion/auto_feed_fetcher.py` - Main fetcher script
2. `ransomeye_intelligence/threat_intel/ingestion/additional_sources.py` - Additional feed sources
3. `systemd/ransomeye-feed-fetcher.service` - Systemd service
4. `systemd/ransomeye-feed-fetcher.timer` - Systemd timer
5. `ransomeye_intelligence/threat_intel/ingestion/INSTALL_AUTO_FETCHER.md` - Installation guide

## Status

✅ **Automatic Fetching:** Daily at 2 AM with hourly retries  
✅ **Internet Detection:** Checks connectivity before fetching  
✅ **Multiple Sources:** 7+ feed sources supported  
✅ **Offline Support:** All feeds cached locally  
✅ **Training Integration:** Automatic integration with training pipelines  
✅ **Monitoring:** Logs and state tracking  

## Next Steps

1. **Install:** Follow installation steps above
2. **Monitor:** Check logs after first run
3. **Verify:** Confirm feeds are being cached
4. **Train:** Use `--use-feeds` flag in training scripts

