// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/state.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Finite state machine tracking - deterministic state transitions

/*
 * Finite State Machine
 * 
 * Tracks correlation state per entity.
 * Explicit transitions only.
 * State corruption → ENGINE HALT
 */

use std::sync::Arc;
use std::collections::HashMap;
use dashmap::DashMap;
use parking_lot::RwLock;
use chrono::{DateTime, Utc};
use tracing::{error, debug, warn};

use crate::errors::CorrelationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
    Initial,
    Reconnaissance,
    Weaponization,
    Delivery,
    Exploitation,
    Installation,
    CommandControl,
    ActionsOnObjectives,
    Alerted,
}

#[derive(Debug, Clone)]
pub struct EntityState {
    pub entity_id: String,
    pub current_state: State,
    pub previous_state: Option<State>,
    pub state_history: Vec<StateTransition>,
    pub entered_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct StateTransition {
    pub from: State,
    pub to: State,
    pub timestamp: DateTime<Utc>,
    pub trigger: String, // Rule ID or event type
}

pub struct StateMachine {
    states: Arc<DashMap<String, RwLock<EntityState>>>,
    valid_transitions: HashMap<State, Vec<State>>,
}

impl StateMachine {
    pub fn new() -> Self {
        let mut valid_transitions = HashMap::new();
        
        // Define valid state transitions (kill-chain progression)
        valid_transitions.insert(State::Initial, vec![State::Reconnaissance]);
        valid_transitions.insert(State::Reconnaissance, vec![State::Weaponization]);
        valid_transitions.insert(State::Weaponization, vec![State::Delivery]);
        valid_transitions.insert(State::Delivery, vec![State::Exploitation]);
        valid_transitions.insert(State::Exploitation, vec![State::Installation]);
        valid_transitions.insert(State::Installation, vec![State::CommandControl]);
        valid_transitions.insert(State::CommandControl, vec![State::ActionsOnObjectives]);
        valid_transitions.insert(State::ActionsOnObjectives, vec![State::Alerted]);
        valid_transitions.insert(State::Alerted, vec![]); // Terminal state
        
        Self {
            states: Arc::new(DashMap::new()),
            valid_transitions,
        }
    }
    
    /// Get or create entity state
    pub fn get_or_create_state(&self, entity_id: &str) -> EntityState {
        let state = self.states
            .entry(entity_id.to_string())
            .or_insert_with(|| {
                RwLock::new(EntityState {
                    entity_id: entity_id.to_string(),
                    current_state: State::Initial,
                    previous_state: None,
                    state_history: Vec::new(),
                    entered_at: Utc::now(),
                    last_updated: Utc::now(),
                })
            });
        
        state.read().clone()
    }
    
    /// Transition entity to new state
    /// Returns Ok(()) on success, CorrelationError on invalid transition
    /// State corruption → ENGINE HALT
    pub fn transition(&self, entity_id: &str, new_state: State, trigger: String) -> Result<(), CorrelationError> {
        let state_lock = self.states
            .entry(entity_id.to_string())
            .or_insert_with(|| {
                RwLock::new(EntityState {
                    entity_id: entity_id.to_string(),
                    current_state: State::Initial,
                    previous_state: None,
                    state_history: Vec::new(),
                    entered_at: Utc::now(),
                    last_updated: Utc::now(),
                })
            });
        
        let mut state = state_lock.write();
        
        // Check if transition is valid
        if !self.is_valid_transition(&state.current_state, &new_state) {
            error!("Invalid state transition for entity {}: {:?} -> {:?}",
                entity_id, state.current_state, new_state);
            return Err(CorrelationError::KillChainStageViolation(
                format!("Invalid transition: {:?} -> {:?}", state.current_state, new_state)
            ));
        }
        
        // Check for state corruption (regression)
        if self.is_state_regression(&state.current_state, &new_state) {
            error!("State corruption detected for entity {}: regression from {:?} to {:?}",
                entity_id, state.current_state, new_state);
            return Err(CorrelationError::StateCorruption(
                format!("State regression: {:?} -> {:?}", state.current_state, new_state)
            ));
        }
        
        // Perform transition
        let transition = StateTransition {
            from: state.current_state.clone(),
            to: new_state.clone(),
            timestamp: Utc::now(),
            trigger,
        };
        
        state.previous_state = Some(state.current_state.clone());
        state.current_state = new_state;
        state.state_history.push(transition.clone());
        state.last_updated = Utc::now();
        
        debug!("State transition for entity {}: {:?} -> {:?}",
            entity_id, transition.from, transition.to);
        
        Ok(())
    }
    
    fn is_valid_transition(&self, from: &State, to: &State) -> bool {
        // Same state is always valid (no-op)
        if from == to {
            return true;
        }
        
        // Check if transition is in valid transitions
        if let Some(valid_targets) = self.valid_transitions.get(from) {
            valid_targets.contains(to)
        } else {
            false
        }
    }
    
    fn is_state_regression(&self, current: &State, new: &State) -> bool {
        // Define state order for regression detection
        let state_order = vec![
            State::Initial,
            State::Reconnaissance,
            State::Weaponization,
            State::Delivery,
            State::Exploitation,
            State::Installation,
            State::CommandControl,
            State::ActionsOnObjectives,
            State::Alerted,
        ];
        
        let current_idx = state_order.iter().position(|s| s == current);
        let new_idx = state_order.iter().position(|s| s == new);
        
        match (current_idx, new_idx) {
            (Some(c), Some(n)) => n < c, // Regression if new index is less than current
            _ => false,
        }
    }
    
    /// Get current state for entity
    pub fn get_state(&self, entity_id: &str) -> Option<State> {
        self.states.get(entity_id)
            .map(|s| s.read().current_state.clone())
    }
    
    /// Get state history for entity
    pub fn get_state_history(&self, entity_id: &str) -> Vec<StateTransition> {
        self.states.get(entity_id)
            .map(|s| s.read().state_history.clone())
            .unwrap_or_default()
    }
    
    /// Reset state for entity (for testing)
    pub fn reset(&self, entity_id: &str) {
        self.states.remove(entity_id);
    }
}

