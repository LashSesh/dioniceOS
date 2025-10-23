//! Common types for the unified system

use core_5d::State5D;

/// Input for cognitive processing (placeholder)
pub struct CognitiveInput {
    pub initial_state: State5D,
}

/// Output from cognitive processing (placeholder)
pub struct CognitiveOutput {
    pub trajectory: Vec<State5D>,
}
