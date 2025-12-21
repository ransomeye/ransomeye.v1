// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/window.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Sliding window management - fixed bounds, deterministic windowing

/*
 * Sliding Window
 * 
 * Manages correlation windows with fixed bounds.
 * No adaptive logic.
 * Window overflow → DROP EVENT
 */

use std::sync::Arc;
use std::collections::VecDeque;
use dashmap::DashMap;
use parking_lot::RwLock;
use chrono::{DateTime, Utc, Duration};
use tracing::{error, debug, warn};

use crate::errors::CorrelationError;

#[derive(Clone, Debug)]
pub struct WindowedEvent {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
}

pub struct SlidingWindow {
    windows: Arc<DashMap<String, RwLock<VecDeque<WindowedEvent>>>>,
    window_size: Duration,
    max_events: usize,
}

impl SlidingWindow {
    pub fn new(window_size_seconds: i64, max_events: usize) -> Self {
        Self {
            windows: Arc::new(DashMap::new()),
            window_size: Duration::seconds(window_size_seconds),
            max_events,
        }
    }
    
    /// Add event to window
    /// Returns Ok(()) on success, CorrelationError on overflow
    pub fn add_event(&self, key: &str, event: WindowedEvent) -> Result<(), CorrelationError> {
        let window = self.windows
            .entry(key.to_string())
            .or_insert_with(|| RwLock::new(VecDeque::new()));
        
        let mut events = window.write();
        
        // Check max events limit
        if events.len() >= self.max_events {
            error!("Window overflow for key: {} (max: {})", key, self.max_events);
            return Err(CorrelationError::WindowOverflow(
                format!("Window overflow: {} events (max: {})", events.len(), self.max_events)
            ));
        }
        
        // Add event
        events.push_back(event.clone());
        
        // Cleanup expired events
        self.cleanup_expired(key, &mut events);
        
        debug!("Event added to window: {} (size: {})", key, events.len());
        Ok(())
    }
    
    /// Get events in window
    pub fn get_events(&self, key: &str) -> Vec<WindowedEvent> {
        let window = self.windows.get(key);
        if let Some(window) = window {
            let events = window.read();
            events.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get events in time window
    pub fn get_events_in_window(&self, key: &str, now: DateTime<Utc>) -> Vec<WindowedEvent> {
        let window = self.windows.get(key);
        if let Some(window) = window {
            let events = window.read();
            let cutoff = now - self.window_size;
            
            events.iter()
                .filter(|e| e.timestamp >= cutoff)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    fn cleanup_expired(&self, key: &str, events: &mut VecDeque<WindowedEvent>) {
        let now = Utc::now();
        let cutoff = now - self.window_size;
        
        while let Some(front) = events.front() {
            if front.timestamp < cutoff {
                events.pop_front();
            } else {
                break;
            }
        }
    }
    
    /// Clear window for key
    pub fn clear(&self, key: &str) {
        self.windows.remove(key);
    }
    
    /// Get window size
    pub fn window_size(&self) -> Duration {
        self.window_size
    }
    
    /// Get max events
    pub fn max_events(&self) -> usize {
        self.max_events
    }
}

