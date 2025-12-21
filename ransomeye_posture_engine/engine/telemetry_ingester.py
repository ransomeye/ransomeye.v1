# Path and File Name : /home/ransomeye/rebuild/ransomeye_posture_engine/engine/telemetry_ingester.py
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details of functionality of this file: Telemetry ingestion from Linux Agent, Windows Agent, and DPI Probe

"""
Telemetry Ingester

Subscribes to telemetry events from:
- Linux Agent
- Windows Agent  
- DPI Probe

Queries database for signed, validated telemetry events.
"""

import asyncio
import logging
from datetime import datetime, timedelta
from typing import List, Dict, Any, Optional
import asyncpg
from dataclasses import dataclass

from ..config import Config

logger = logging.getLogger("ransomeye_posture_engine.ingester")

@dataclass
class TelemetryEvent:
    """Normalized telemetry event."""
    event_id: str
    producer_type: str  # "linux_agent", "windows_agent", "dpi_probe"
    host_id: Optional[str]
    timestamp: datetime
    event_type: str
    data: Dict[str, Any]
    signature_valid: bool

class TelemetryIngester:
    """Ingests telemetry from agents and DPI probe."""
    
    def __init__(self, config: Config):
        self.config = config
        self.db_pool: Optional[asyncpg.Pool] = None
        self.last_poll_time: Optional[datetime] = None
        
    async def initialize(self):
        """Initialize database connection pool."""
        try:
            self.db_pool = await asyncpg.create_pool(
                host=self.config.db_host,
                port=self.config.db_port,
                database=self.config.db_name,
                user=self.config.db_user,
                password=self.config.db_pass,
                min_size=2,
                max_size=10,
            )
            logger.info("Database connection pool initialized")
        except Exception as e:
            logger.error(f"Failed to initialize database pool: {e}")
            raise
    
    async def close(self):
        """Close database connection pool."""
        if self.db_pool:
            await self.db_pool.close()
            logger.info("Database connection pool closed")
    
    async def fetch_new_telemetry(self, since: Optional[datetime] = None) -> List[TelemetryEvent]:
        """
        Fetch new telemetry events from database.
        
        Args:
            since: Only fetch events after this timestamp. If None, uses last poll time.
        
        Returns:
            List of normalized telemetry events.
        """
        if not self.db_pool:
            raise RuntimeError("Database pool not initialized")
        
        # Determine query start time
        if since:
            query_since = since
        elif self.last_poll_time:
            query_since = self.last_poll_time
        else:
            # First poll - get last hour
            query_since = datetime.utcnow() - timedelta(hours=1)
        
        events = []
        
        try:
            async with self.db_pool.acquire() as conn:
                # Query telemetry table (assuming standard schema)
                # Adjust table/column names based on actual schema
                query = """
                    SELECT 
                        event_id,
                        producer_id,
                        producer_type,
                        host_id,
                        timestamp,
                        event_type,
                        event_data,
                        signature_valid
                    FROM telemetry_events
                    WHERE timestamp > $1
                        AND signature_valid = true
                        AND producer_type IN ('linux_agent', 'windows_agent', 'dpi_probe')
                    ORDER BY timestamp ASC
                """
                
                rows = await conn.fetch(query, query_since)
                
                for row in rows:
                    event = TelemetryEvent(
                        event_id=row['event_id'],
                        producer_type=row['producer_type'],
                        host_id=row['host_id'],
                        timestamp=row['timestamp'],
                        event_type=row['event_type'],
                        data=row['event_data'] if isinstance(row['event_data'], dict) 
                            else row['event_data'],
                        signature_valid=row['signature_valid'],
                    )
                    events.append(event)
                
                logger.info(f"Fetched {len(events)} telemetry events since {query_since}")
                
        except Exception as e:
            logger.error(f"Error fetching telemetry: {e}")
            # Fail-closed: re-raise on database errors
            raise
        
        # Update last poll time
        self.last_poll_time = datetime.utcnow()
        
        return events
    
    async def get_host_telemetry(self, host_id: str, 
                                start_time: datetime, 
                                end_time: datetime) -> List[TelemetryEvent]:
        """
        Get telemetry for a specific host within a time range.
        
        Args:
            host_id: Host identifier
            start_time: Start of time range
            end_time: End of time range
        
        Returns:
            List of telemetry events for the host.
        """
        if not self.db_pool:
            raise RuntimeError("Database pool not initialized")
        
        events = []
        
        try:
            async with self.db_pool.acquire() as conn:
                query = """
                    SELECT 
                        event_id,
                        producer_id,
                        producer_type,
                        host_id,
                        timestamp,
                        event_type,
                        event_data,
                        signature_valid
                    FROM telemetry_events
                    WHERE host_id = $1
                        AND timestamp >= $2
                        AND timestamp <= $3
                        AND signature_valid = true
                    ORDER BY timestamp ASC
                """
                
                rows = await conn.fetch(query, host_id, start_time, end_time)
                
                for row in rows:
                    event = TelemetryEvent(
                        event_id=row['event_id'],
                        producer_type=row['producer_type'],
                        host_id=row['host_id'],
                        timestamp=row['timestamp'],
                        event_type=row['event_type'],
                        data=row['event_data'] if isinstance(row['event_data'], dict) 
                            else row['event_data'],
                        signature_valid=row['signature_valid'],
                    )
                    events.append(event)
                
        except Exception as e:
            logger.error(f"Error fetching host telemetry: {e}")
            raise
        
        return events

