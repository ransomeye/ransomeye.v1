// Path and File Name : /home/ransomeye/rebuild/ransomeye_dpi_probe/tests/backpressure_tests.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Tests for backpressure handling and buffer management

use ransomeye_dpi_probe::backpressure::BackpressureHandler;
use std::sync::Arc;

#[test]
fn test_backpressure_activation() {
    let handler = BackpressureHandler::new(1024, 512);
    
    // Initially no backpressure
    assert!(!handler.is_backpressure_active());
    assert!(handler.can_accept());
    
    // Set backpressure
    handler.set_backpressure(true);
    assert!(handler.is_backpressure_active());
    
    // Clear backpressure
    handler.set_backpressure(false);
    assert!(!handler.is_backpressure_active());
}

#[test]
fn test_buffer_size_tracking() {
    let handler = BackpressureHandler::new(1024, 512);
    
    // Initial size is zero
    assert_eq!(handler.get_buffer_size(), 0);
    
    // Increment buffer
    assert!(handler.increment_buffer(100));
    assert_eq!(handler.get_buffer_size(), 100);
    
    // Decrement buffer
    handler.decrement_buffer(50);
    assert_eq!(handler.get_buffer_size(), 50);
}

#[test]
fn test_backpressure_threshold() {
    let handler = BackpressureHandler::new(1024, 512);
    
    // Below threshold
    handler.increment_buffer(256);
    assert!(!handler.should_backpressure());
    
    // At threshold
    handler.increment_buffer(256); // Total: 512
    assert!(handler.should_backpressure());
    
    // Above threshold
    handler.increment_buffer(100); // Total: 612
    assert!(handler.should_backpressure());
}

#[test]
fn test_buffer_capacity_limit() {
    let handler = BackpressureHandler::new(1024, 512);
    
    // Fill buffer to capacity
    assert!(handler.increment_buffer(1023));
    assert!(handler.can_accept());
    
    // Exceed capacity
    assert!(!handler.increment_buffer(2)); // Would exceed 1024
    assert!(!handler.can_accept());
}

#[test]
fn test_drop_counting() {
    let handler = BackpressureHandler::new(1024, 512);
    
    // Initially no drops
    assert_eq!(handler.get_dropped_count(), 0);
    
    // Exceed capacity (should increment drop count)
    handler.increment_buffer(1024);
    handler.increment_buffer(1); // This should fail and increment drops
    
    // Drop count should be incremented
    let drops = handler.get_dropped_count();
    assert!(drops > 0);
    
    // Reset stats
    handler.reset_stats();
    assert_eq!(handler.get_dropped_count(), 0);
}

#[test]
fn test_concurrent_access() {
    use std::sync::Arc;
    use std::thread;
    
    let handler = Arc::new(BackpressureHandler::new(10000, 5000));
    
    // Spawn multiple threads to increment/decrement buffer
    let mut handles = vec![];
    for _ in 0..10 {
        let h = handler.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                h.increment_buffer(10);
                h.decrement_buffer(10);
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Buffer should be back to zero (all increments/decrements matched)
    assert_eq!(handler.get_buffer_size(), 0);
}
