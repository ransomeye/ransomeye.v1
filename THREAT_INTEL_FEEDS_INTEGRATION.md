# Threat Intelligence Feeds Integration

**Path and File Name:** `/home/ransomeye/rebuild/THREAT_INTEL_FEEDS_INTEGRATION.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Integration of threat intelligence feeds into RansomEye AI/ML/LLM training pipelines

## Overview

Threat intelligence feeds have been integrated into RansomEye's AI/ML/LLM training pipelines to enhance training data with real-world threat patterns while maintaining Phase 6 compliance (synthetic data generation, SHAP explainability, fail-closed governance).

## Integrated Feeds

### 1. MalwareBazaar
- **API:** https://mb-api.abuse.ch/api/v1/
- **Auth-Key:** `483ce60ba7c8a3d7358e3c8afd6e6d23a746eb2a5a42479f`
- **Collector:** `ransomeye_intelligence/threat_intel/ingestion/malwarebazaar_feed.py`
- **Cache:** `/home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/cache/malwarebazaar/`

### 2. Wiz.io Cloud Threat Landscape
- **API:** https://www.wiz.io/api/feed/cloud-threat-landscape/stix.json
- **Format:** STIX 2.1 JSON
- **Collector:** `ransomeye_intelligence/threat_intel/ingestion/wiz_feed.py`
- **Cache:** `/home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/cache/wiz/`

### 3. Ransomware.live
- **API:** https://api.ransomware.live/v1
- **API Key:** `6c0cca08-3419-43e6-8014-0a4f87f353a3`
- **Collector:** `ransomeye_intelligence/threat_intel/ingestion/ransomware_live_feed.py`
- **Cache:** `/home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/cache/ransomware_live/`

## Usage

### Step 1: Fetch and Cache Feeds

```bash
export MALWARBAZAAR_AUTH_KEY="483ce60ba7c8a3d7358e3c8afd6e6d23a746eb2a5a42479f"
export RANSOMWARE_LIVE_API_KEY="6c0cca08-3419-43e6-8014-0a4f87f353a3"

# Fetch all feeds
python3 -m ransomeye_intelligence.threat_intel.ingestion.fetch_all_feeds
```

### Step 2: Train with Enhanced Data

```bash
# Train with synthetic + threat intelligence feeds
python3 ransomeye_intelligence/baseline_pack/train_baseline_models.py --use-feeds

# Train with threat intelligence feeds only
python3 ransomeye_intelligence/baseline_pack/train_baseline_models.py --feed-only
```

### Step 3: Offline Training

Once feeds are cached, training can proceed completely offline:

```bash
# Load from cache only (no network access)
python3 -m ransomeye_intelligence.threat_intel.ingestion.fetch_all_feeds --cache-only

# Train offline
python3 ransomeye_intelligence/baseline_pack/train_baseline_models.py --use-feeds
```

## Integration Architecture

```
Threat Intelligence Feeds
    ↓
Feed Collectors (malwarebazaar_feed.py, wiz_feed.py, ransomware_live_feed.py)
    ↓
Local Cache (offline-capable)
    ↓
Enhanced Training Data Generator (enhance_training_with_feeds.py)
    ↓
Training Pipeline (train_baseline_models.py)
    ↓
Trained Models with SHAP + Signing
```

## Phase 6 Compliance

✅ **Training Pipelines:** All models have training scripts with incremental learning support  
✅ **SHAP Explainability:** Maintained for all numeric inferences  
✅ **Model Signing:** Ed25519 signing support added  
✅ **Resource Governance:** CPU/memory limits enforced  
✅ **Offline Operation:** All feeds cached locally  
✅ **Synthetic Data:** Feeds enhance synthetic data, not replace it  

## Files Created

1. `ransomeye_intelligence/threat_intel/ingestion/malwarebazaar_feed.py` - MalwareBazaar collector
2. `ransomeye_intelligence/threat_intel/ingestion/wiz_feed.py` - Wiz.io STIX collector
3. `ransomeye_intelligence/threat_intel/ingestion/ransomware_live_feed.py` - Ransomware.live collector
4. `ransomeye_intelligence/threat_intel/ingestion/fetch_all_feeds.py` - Unified feed fetcher
5. `ransomeye_intelligence/baseline_pack/enhance_training_with_feeds.py` - Enhanced training data generator
6. `ransomeye_intelligence/threat_intel/ingestion/README.md` - Documentation

## Data Privacy

- ✅ All feeds cached locally
- ✅ No customer data used
- ✅ Feeds used only for training data enhancement
- ✅ All training data is synthetic or from public threat intelligence
- ✅ Offline training capability maintained

## Next Steps

1. **Initial Feed Fetch:** Run `fetch_all_feeds.py` to populate cache
2. **Enhanced Training:** Use `--use-feeds` flag in training scripts
3. **Periodic Updates:** Re-run `fetch_all_feeds.py` periodically to update cached feeds
4. **Offline Training:** Use cached feeds for completely offline training

## Environment Variables

```bash
export MALWARBAZAAR_AUTH_KEY="483ce60ba7c8a3d7358e3c8afd6e6d23a746eb2a5a42479f"
export RANSOMWARE_LIVE_API_KEY="6c0cca08-3419-43e6-8014-0a4f87f353a3"
```

## Testing

```bash
# Test MalwareBazaar collector
python3 -m ransomeye_intelligence.threat_intel.ingestion.malwarebazaar_feed --limit 10

# Test Wiz.io collector
python3 -m ransomeye_intelligence.threat_intel.ingestion.wiz_feed

# Test Ransomware.live collector
export RANSOMWARE_LIVE_API_KEY="6c0cca08-3419-43e6-8014-0a4f87f353a3"
python3 -m ransomeye_intelligence.threat_intel.ingestion.ransomware_live_feed --limit 10
```

## Status

✅ **Integration Complete:** All three feeds integrated  
✅ **Offline Support:** All feeds cached locally  
✅ **Training Integration:** Enhanced data generator created  
✅ **Phase 6 Compliant:** All requirements met  

