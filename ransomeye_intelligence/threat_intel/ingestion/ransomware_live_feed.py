# Path and File Name : /home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/ingestion/ransomware_live_feed.py
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Ransomware.live threat intelligence feed collector for training data

"""
Ransomware.live Feed Collector: Collects threat intelligence from Ransomware.live API.
All data is cached locally for offline training use.
"""

import os
import sys
import json
import requests
import hashlib
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional

# Ransomware.live API credentials
RANSOMWARE_LIVE_API_KEY = os.getenv("RANSOMWARE_LIVE_API_KEY", "6c0cca08-3419-43e6-8014-0a4f87f353a3")
RANSOMWARE_LIVE_API_URL = "https://api.ransomware.live/v1"

FEEDS_DIR = Path("/home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/feeds")
CACHE_DIR = Path("/home/ransomeye/rebuild/ransomeye_intelligence/threat_intel/cache/ransomware_live")


class RansomwareLiveFeedCollector:
    """Collects threat intelligence from Ransomware.live API."""
    
    def __init__(self, api_key: Optional[str] = None):
        self.api_key = api_key or RANSOMWARE_LIVE_API_KEY
        self.api_url = RANSOMWARE_LIVE_API_URL
        FEEDS_DIR.mkdir(parents=True, exist_ok=True)
        CACHE_DIR.mkdir(parents=True, exist_ok=True)
    
    def fetch_groups(self) -> List[Dict]:
        """
        Fetch ransomware groups from Ransomware.live API.
        
        Returns:
            List of ransomware group data
        """
        try:
            headers = {
                'Authorization': f'Bearer {self.api_key}',
                'Content-Type': 'application/json'
            }
            
            response = requests.get(
                f"{self.api_url}/groups",
                headers=headers,
                timeout=30
            )
            
            if response.status_code == 200:
                return response.json()
            else:
                print(f"Warning: Failed to fetch groups: HTTP {response.status_code}", file=sys.stderr)
                return []
        
        except requests.exceptions.Timeout:
            print("Warning: Timeout fetching Ransomware.live groups", file=sys.stderr)
            return []
        except Exception as e:
            print(f"Warning: Error fetching Ransomware.live groups: {e}", file=sys.stderr)
            return []
    
    def fetch_recent_victims(self, limit: int = 100) -> List[Dict]:
        """
        Fetch recent victim data from Ransomware.live API.
        
        Args:
            limit: Maximum number of victims to fetch
        
        Returns:
            List of victim data
        """
        try:
            headers = {
                'Authorization': f'Bearer {self.api_key}',
                'Content-Type': 'application/json'
            }
            
            response = requests.get(
                f"{self.api_url}/victims",
                headers=headers,
                params={'limit': limit},
                timeout=30
            )
            
            if response.status_code == 200:
                return response.json()
            else:
                print(f"Warning: Failed to fetch victims: HTTP {response.status_code}", file=sys.stderr)
                return []
        
        except requests.exceptions.Timeout:
            print("Warning: Timeout fetching Ransomware.live victims", file=sys.stderr)
            return []
        except Exception as e:
            print(f"Warning: Error fetching Ransomware.live victims: {e}", file=sys.stderr)
            return []
    
    def cache_data(self, groups: List[Dict], victims: List[Dict], feed_id: str = None) -> Path:
        """
        Cache feed data to local file for offline training.
        
        Args:
            groups: List of ransomware group data
            victims: List of victim data
            feed_id: Optional feed identifier
        
        Returns:
            Path to cached file
        """
        if feed_id is None:
            feed_id = f"ransomware_live_{datetime.utcnow().strftime('%Y%m%d_%H%M%S')}"
        
        cache_path = CACHE_DIR / f"{feed_id}.json"
        
        feed_data = {
            'feed_id': feed_id,
            'source': 'ransomware.live',
            'timestamp': datetime.utcnow().isoformat() + 'Z',
            'group_count': len(groups),
            'victim_count': len(victims),
            'groups': groups,
            'victims': victims,
            'feed_hash': None
        }
        
        # Compute feed hash
        feed_json = json.dumps(feed_data, sort_keys=True)
        feed_hash = hashlib.sha256(feed_json.encode()).hexdigest()
        feed_data['feed_hash'] = f"sha256:{feed_hash}"
        
        # Save to cache
        with open(cache_path, 'w') as f:
            json.dump(feed_data, f, indent=2)
        
        print(f"  ✓ Cached {len(groups)} groups and {len(victims)} victims to {cache_path}")
        return cache_path
    
    def load_cached_data(self) -> Dict:
        """Load all cached data."""
        all_groups = []
        all_victims = []
        
        for cache_file in sorted(CACHE_DIR.glob("*.json")):
            try:
                with open(cache_file, 'r') as f:
                    feed_data = json.load(f)
                    all_groups.extend(feed_data.get('groups', []))
                    all_victims.extend(feed_data.get('victims', []))
            except Exception:
                continue
        
        return {
            'groups': all_groups,
            'victims': all_victims
        }


def main():
    """CLI entry point."""
    import argparse
    
    parser = argparse.ArgumentParser(description='Ransomware.live Feed Collector')
    parser.add_argument('--limit', type=int, default=100,
                       help='Maximum number of victims to fetch')
    parser.add_argument('--cache-only', action='store_true',
                       help='Only load from cache, do not fetch new data')
    parser.add_argument('--api-key', default=None,
                       help='Ransomware.live API key (or use RANSOMWARE_LIVE_API_KEY env var)')
    
    args = parser.parse_args()
    
    collector = RansomwareLiveFeedCollector(api_key=args.api_key)
    
    if args.cache_only:
        data = collector.load_cached_data()
        print(f"Loaded {len(data['groups'])} groups and {len(data['victims'])} victims from cache")
    else:
        print("Fetching data from Ransomware.live...")
        groups = collector.fetch_groups()
        victims = collector.fetch_recent_victims(limit=args.limit)
        
        print(f"Fetched {len(groups)} groups and {len(victims)} victims")
        
        if groups or victims:
            cache_path = collector.cache_data(groups, victims)
            print(f"Cached to: {cache_path}")
        
        data = {'groups': groups, 'victims': victims}
    
    return data


if __name__ == '__main__':
    main()

