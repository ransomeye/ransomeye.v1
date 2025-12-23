# Quick Start: Threat Intelligence Feeds

**Use `python3` (not `python`) on this system**

## Step 1: Set Environment Variables

```bash
export MALWARBAZAAR_AUTH_KEY="483ce60ba7c8a3d7358e3c8afd6e6d23a746eb2a5a42479f"
export RANSOMWARE_LIVE_API_KEY="6c0cca08-3419-43e6-8014-0a4f87f353a3"
```

## Step 2: Fetch and Cache All Feeds

```bash
python3 -m ransomeye_intelligence.threat_intel.ingestion.fetch_all_feeds
```

**Expected Output:**
- ✓ MalwareBazaar: 100 samples cached
- ✓ Wiz.io: 356 IOCs cached  
- ✓ Ransomware.live: 306 groups cached

## Step 3: Train with Enhanced Data

```bash
python3 ransomeye_intelligence/baseline_pack/train_baseline_models.py --use-feeds
```

## Offline Training (After Initial Fetch)

```bash
# Load from cache only (no network access)
python3 -m ransomeye_intelligence.threat_intel.ingestion.fetch_all_feeds --cache-only

# Train offline
python3 ransomeye_intelligence/baseline_pack/train_baseline_models.py --use-feeds
```

## Individual Feed Testing

```bash
# Test MalwareBazaar
python3 -m ransomeye_intelligence.threat_intel.ingestion.malwarebazaar_feed --limit 10

# Test Wiz.io
python3 -m ransomeye_intelligence.threat_intel.ingestion.wiz_feed

# Test Ransomware.live
python3 -m ransomeye_intelligence.threat_intel.ingestion.ransomware_live_feed --limit 10
```

## Cache Locations

- MalwareBazaar: `/home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/cache/malwarebazaar/`
- Wiz.io: `/home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/cache/wiz/`
- Ransomware.live: `/home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/cache/ransomware_live/`

