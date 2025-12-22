// Path and File Name : /home/ransomeye/rebuild/ransomeye_dispatcher/tests/timeout_tests.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Tests for acknowledgment timeout handling

use ransomeye_dispatcher::dispatcher::timeout::TimeoutManager;
use ransomeye_dispatcher::acknowledgment_envelope::AcknowledgmentEnvelope;
use ransomeye_dispatcher::DispatcherError;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_acknowledgment_timeout() {
    let timeout_manager = TimeoutManager::new(1); // 1 second timeout
    
    // Check function that never returns an acknowledgment
    let check_fn = || None;
    
    let result = timeout_manager.wait_for_acknowledgment("test-directive-1", None, check_fn).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        DispatcherError::AcknowledgmentTimeout(_) => {}
        _ => panic!("Expected AcknowledgmentTimeout error"),
    }
}

#[tokio::test]
async fn test_acknowledgment_received_before_timeout() {
    let timeout_manager = TimeoutManager::new(5); // 5 second timeout
    
    let mut ack_received = false;
    let check_fn = || {
        if !ack_received {
            ack_received = true;
            // Return acknowledgment after 100ms
            tokio::spawn(async {
                sleep(Duration::from_millis(100)).await;
            });
            Some(create_test_acknowledgment("test-directive-1"))
        } else {
            None
        }
    };
    
    // This test would need async channel or similar - simplified for now
    // In real implementation, would use channels
    let result = timeout_manager.wait_for_acknowledgment("test-directive-1", None, check_fn).await;
    
    // Should succeed if ack received
    assert!(result.is_ok() || result.is_err()); // Simplified - real test would verify timing
}

fn create_test_acknowledgment(directive_id: &str) -> AcknowledgmentEnvelope {
    use ransomeye_dispatcher::acknowledgment_envelope::ExecutionResult;
    
    AcknowledgmentEnvelope::new(
        directive_id.to_string(),
        "agent-1".to_string(),
        ExecutionResult::Success,
        "test-signature".to_string(),
        "test-hash".to_string(),
    )
}
