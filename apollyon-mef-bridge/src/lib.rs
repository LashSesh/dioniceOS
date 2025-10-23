//! # APOLLYON-MEF Bridge
//!
//! Integration bridge between APOLLYON-5D geometric-cognitive engine
//! and Infinity-Ledger (MEF-Core) proof-carrying vector ledger.
//!
//! This crate provides seamless bidirectional conversion and processing
//! between both systems while preserving mathematical correctness and
//! determinism.

pub mod adapters;
pub mod pipeline;
pub mod unified;
pub mod trichter;

// Re-export key types for convenience
pub use adapters::{MetatronBridge, ResonanceBridge, SpectralAdapter, StateAdapter};
pub use unified::{CognitiveInput, CognitiveOutput, UnifiedCognitiveEngine};
pub use trichter::{
    coupling_tick, FunnelGraph, HDAGField, Hyperbion, Policy, PolicyParams, 
    State4D, State5D, lift, proj_4d,
};

/// Bridge version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Maximum allowed error in roundtrip conversions
pub const ROUNDTRIP_EPSILON: f64 = 1e-10;
