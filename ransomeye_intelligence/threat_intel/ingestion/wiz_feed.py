# Path and File Name : /home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/ingestion/wiz_feed.py
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Wiz.io Cloud Threat Landscape STIX feed collector for training data

"""
Wiz.io Feed Collector: Collects threat intelligence from Wiz.io Cloud Threat Landscape STIX feed.
All data is cached locally for offline training use.
"""

import os
import sys
import json
import subprocess
import hashlib
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional

# Wiz.io API endpoint
WIZ_API_URL = "https://www.wiz.io/api/feed/cloud-threat-landscape/stix.json"

FEEDS_DIR = Path("/home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/feeds")
CACHE_DIR = Path("/home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/cache/wiz")


class WizFeedCollector:
    """Collects threat intelligence from Wiz.io STIX feed."""
    
    def __init__(self):
        self.api_url = WIZ_API_URL
        FEEDS_DIR.mkdir(parents=True, exist_ok=True)
        CACHE_DIR.mkdir(parents=True, exist_ok=True)
    
    def fetch_stix_feed(self) -> Optional[Dict]:
        """
        Fetch STIX feed from Wiz.io.
        
        Returns:
            STIX feed data or None if fetch failed
        """
        try:
            # Use wget to fetch STIX feed
            cmd = [
                'wget',
                self.api_url,
                '-O-',
                '--timeout=60',
                '--tries=2'
            ]
            
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=60
            )
            
            if result.returncode != 0:
                print(f"Warning: Failed to fetch Wiz.io feed: {result.stderr}", file=sys.stderr)
                return None
            
            try:
                data = json.loads(result.stdout)
                return data
            except json.JSONDecodeError:
                print("Warning: Invalid JSON response from Wiz.io", file=sys.stderr)
                return None
        
        except subprocess.TimeoutExpired:
            print("Warning: Timeout fetching Wiz.io feed", file=sys.stderr)
            return None
        except Exception as e:
            print(f"Warning: Error fetching Wiz.io feed: {e}", file=sys.stderr)
            return None
    
    def parse_stix_objects(self, stix_data: Dict) -> List[Dict]:
        """
        Parse STIX objects and extract IOCs.
        
        Args:
            stix_data: STIX JSON data
        
        Returns:
            List of parsed IOCs
        """
        iocs = []
        
        # STIX 2.1 format: objects array
        objects = stix_data.get('objects', [])
        
        for obj in objects:
            obj_type = obj.get('type', '')
            
            # Extract indicators
            if obj_type == 'indicator':
                pattern = obj.get('pattern', '')
                labels = obj.get('labels', [])
                
                ioc = {
                    'type': 'indicator',
                    'pattern': pattern,
                    'labels': labels,
                    'created': obj.get('created'),
                    'modified': obj.get('modified'),
                    'description': obj.get('description', ''),
                    'confidence': obj.get('confidence', 0.5),
                    'source': 'wiz.io'
                }
                iocs.append(ioc)
            
            # Extract threat actors
            elif obj_type == 'threat-actor':
                ioc = {
                    'type': 'threat-actor',
                    'name': obj.get('name', ''),
                    'description': obj.get('description', ''),
                    'labels': obj.get('labels', []),
                    'created': obj.get('created'),
                    'modified': obj.get('modified'),
                    'source': 'wiz.io'
                }
                iocs.append(ioc)
            
            # Extract malware
            elif obj_type == 'malware':
                ioc = {
                    'type': 'malware',
                    'name': obj.get('name', ''),
                    'description': obj.get('description', ''),
                    'labels': obj.get('labels', []),
                    'created': obj.get('created'),
                    'modified': obj.get('modified'),
                    'source': 'wiz.io'
                }
                iocs.append(ioc)
        
        return iocs
    
    def cache_feed(self, stix_data: Dict, feed_id: str = None) -> Path:
        """
        Cache STIX feed to local file for offline training.
        
        Args:
            stix_data: STIX feed data
            feed_id: Optional feed identifier
        
        Returns:
            Path to cached file
        """
        if feed_id is None:
            feed_id = f"wiz_{datetime.utcnow().strftime('%Y%m%d_%H%M%S')}"
        
        cache_path = CACHE_DIR / f"{feed_id}.json"
        
        # Parse IOCs
        iocs = self.parse_stix_objects(stix_data)
        
        feed_data = {
            'feed_id': feed_id,
            'source': 'wiz.io',
            'format': 'stix',
            'timestamp': datetime.utcnow().isoformat() + 'Z',
            'ioc_count': len(iocs),
            'iocs': iocs,
            'raw_stix': stix_data,
            'feed_hash': None
        }
        
        # Compute feed hash
        feed_json = json.dumps(feed_data, sort_keys=True)
        feed_hash = hashlib.sha256(feed_json.encode()).hexdigest()
        feed_data['feed_hash'] = f"sha256:{feed_hash}"
        
        # Save to cache
        with open(cache_path, 'w') as f:
            json.dump(feed_data, f, indent=2)
        
        print(f"  ✓ Cached {len(iocs)} IOCs to {cache_path}")
        return cache_path
    
    def load_cached_feeds(self) -> List[Dict]:
        """Load all cached feeds."""
        all_iocs = []
        
        for cache_file in sorted(CACHE_DIR.glob("*.json")):
            try:
                with open(cache_file, 'r') as f:
                    feed_data = json.load(f)
                    iocs = feed_data.get('iocs', [])
                    all_iocs.extend(iocs)
            except Exception:
                continue
        
        return all_iocs


def main():
    """CLI entry point."""
    import argparse
    
    parser = argparse.ArgumentParser(description='Wiz.io Feed Collector')
    parser.add_argument('--cache-only', action='store_true',
                       help='Only load from cache, do not fetch new data')
    
    args = parser.parse_args()
    
    collector = WizFeedCollector()
    
    if args.cache_only:
        iocs = collector.load_cached_feeds()
        print(f"Loaded {len(iocs)} IOCs from cache")
    else:
        print("Fetching STIX feed from Wiz.io...")
        stix_data = collector.fetch_stix_feed()
        
        if stix_data:
            iocs = collector.parse_stix_objects(stix_data)
            print(f"Parsed {len(iocs)} IOCs from STIX feed")
            
            cache_path = collector.cache_feed(stix_data)
            print(f"Cached to: {cache_path}")
        else:
            print("Failed to fetch feed")
            iocs = []
    
    return iocs


if __name__ == '__main__':
    main()

