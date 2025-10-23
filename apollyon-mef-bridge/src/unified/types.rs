//! Common types for the unified system

use core_5d::{State5D, SystemParameters};
use mef_schemas::{GateDecision, KnowledgeObject, RouteSpec, SpectralSignature};

use crate::adapters::resonance_adapter::ProofOfResonanceData;

/// Input for cognitive processing
///
/// Encapsulates all parameters needed to run the complete APOLLYON → MEF pipeline
#[derive(Clone)]
pub struct CognitiveInput {
    /// Initial 5D state for APOLLYON integration
    pub initial_state: State5D,

    /// System parameters for APOLLYON dynamics
    pub parameters: SystemParameters,

    /// Final integration time
    pub t_final: f64,

    /// TIC identifier for MEF storage
    pub tic_id: String,

    /// Seed for route selection
    pub seed: String,

    /// HD-style seed derivation path (e.g., "MEF/domain/stage/0001")
    pub seed_path: String,
}

/// Output from cognitive processing
///
/// Contains all results from the unified APOLLYON + MEF pipeline
pub struct CognitiveOutput {
    /// Final 5D trajectory from APOLLYON integration
    pub trajectory: Vec<State5D>,

    /// Spectral signature computed from trajectory
    pub spectral_signature: SpectralSignature,

    /// Selected MEF route
    pub route: RouteSpec,

    /// Proof-of-Resonance data
    pub proof: ProofOfResonanceData,

    /// Gate decision (FIRE or HOLD)
    pub gate_decision: GateDecision,

    /// Knowledge object (if created)
    pub knowledge: Option<KnowledgeObject>,
}
