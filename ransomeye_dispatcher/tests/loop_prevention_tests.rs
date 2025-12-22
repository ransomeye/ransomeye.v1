// Path and File Name : /home/ransomeye/rebuild/ransomeye_dispatcher/tests/loop_prevention_tests.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Tests for loop prevention and reentrancy protection

use ransomeye_dispatcher::dispatcher::reentrancy::ReentrancyGuard;
use ransomeye_dispatcher::DispatcherError;

#[test]
fn test_reentrancy_detected() {
    let guard = ReentrancyGuard::new();
    
    // First entry should succeed
    let _token1 = guard.enter("directive-1").unwrap();
    
    // Second entry (reentrancy) should fail
    let result = guard.enter("directive-2");
    assert!(result.is_err());
    match result.unwrap_err() {
        DispatcherError::ReentrancyDetected => {}
        _ => panic!("Expected ReentrancyDetected error"),
    }
}

#[test]
fn test_loop_detected() {
    let guard = ReentrancyGuard::new();
    
    // Enter with directive-1
    let _token1 = guard.enter("directive-1").unwrap();
    
    // Try to enter again with same directive (loop)
    let result = guard.enter("directive-1");
    assert!(result.is_err());
    match result.unwrap_err() {
        DispatcherError::LoopDetected => {}
        _ => panic!("Expected LoopDetected error"),
    }
}

#[test]
fn test_guard_released_after_drop() {
    let guard = ReentrancyGuard::new();
    
    {
        let _token = guard.enter("directive-1").unwrap();
        // Token dropped here
    }
    
    // Should be able to enter again after token dropped
    let _token2 = guard.enter("directive-1").unwrap();
}
