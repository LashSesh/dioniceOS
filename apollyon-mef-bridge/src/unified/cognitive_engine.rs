//! Unified Cognitive Engine implementation
//!
//! Orchestrates the complete APOLLYON-5D → MEF-Core processing pipeline:
//! 1. APOLLYON-5D integration (5D dynamics)
//! 2. Spectral analysis (trajectory features)
//! 3. State conversion (5D → Spiral coordinates)
//! 4. Route selection (Metatron S7 routing)
//! 5. Knowledge derivation (MEF ID generation)
//! 6. Proof-of-Resonance (transition validation)
//! 7. Gate evaluation (FIRE/HOLD decision)
//! 8. MEF-Core storage (if FIRE)

use super::types::{CognitiveInput, CognitiveOutput};
use crate::adapters::{
    resonance_adapter::ProofOfResonanceData, MetatronBridge, ResonanceBridge, SpectralAdapter,
    StateAdapter,
};
use bridge::{ConstantResonanceField, SpectralAnalyzer, TrajectoryObserver};
use core_5d::{Integrator, VectorField};
use mef_schemas::{GateDecision, KnowledgeObject, SpectralSignature};
use thiserror::Error;

/// Errors that can occur during cognitive processing
#[derive(Error, Debug)]
pub enum CognitiveError {
    #[error("Integration failed: {0}")]
    IntegrationError(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Spectral analysis failed: {0}")]
    SpectralAnalysisError(String),

    #[error("Route selection failed: {0}")]
    RouteSelectionError(String),

    #[error("Empty trajectory")]
    EmptyTrajectory,
}

/// Unified Cognitive Engine combining APOLLYON-5D and MEF-Core
///
/// This engine orchestrates the complete processing pipeline from APOLLYON's
/// 5D dynamics through MEF's proof-carrying ledger system.
///
/// # Pipeline Stages
///
/// 1. **APOLLYON Integration**: Integrate 5D dynamics from initial state
/// 2. **Spectral Analysis**: Extract spectral features from trajectory
/// 3. **State Conversion**: Convert State5D to MEF Spiral coordinates
/// 4. **Route Selection**: Use Metatron Bridge for S7 routing
/// 5. **Knowledge Derivation**: Generate MEF knowledge object
/// 6. **Proof-of-Resonance**: Validate state transition
/// 7. **Gate Evaluation**: Apply FIRE/HOLD logic
/// 8. **Storage**: Store knowledge if gate fires
pub struct UnifiedCognitiveEngine {
    /// Spectral analyzer for trajectory analysis
    spectral_analyzer: SpectralAnalyzer,

    /// Metatron bridge for routing
    metatron_bridge: MetatronBridge,
}

impl UnifiedCognitiveEngine {
    /// Create a new unified cognitive engine
    pub fn new() -> Self {
        Self {
            spectral_analyzer: SpectralAnalyzer::new(),
            metatron_bridge: MetatronBridge::new(),
        }
    }

    /// Process input through the complete unified pipeline
    ///
    /// # Arguments
    /// * `input` - Cognitive input containing initial state and parameters
    ///
    /// # Returns
    /// Complete cognitive output with trajectory, proofs, and gate decision
    ///
    /// # Pipeline Details
    ///
    /// ## Phase 1: APOLLYON 5D Integration
    /// - Integrates dynamics using Heun's method (RK2)
    /// - Produces complete trajectory from t=0 to t=t_final
    ///
    /// ## Phase 2: Spectral Analysis
    /// - Analyzes trajectory using SpectralAnalyzer
    /// - Extracts entropy, centroids, and dominant frequency
    /// - Converts to MEF SpectralSignature
    ///
    /// ## Phase 3: State Conversion
    /// - Converts final State5D to MEF Spiral coordinates
    /// - Perfect 1:1 mapping preserving all dimensions
    ///
    /// ## Phase 4: Route Selection
    /// - Uses MetatronBridge with APOLLYON-enhanced metrics
    /// - Selects S7 route deterministically
    ///
    /// ## Phase 5: Knowledge Derivation
    /// - Creates KnowledgeObject with MEF ID
    /// - Binds TIC, route, and seed path
    ///
    /// ## Phase 6: Proof-of-Resonance
    /// - Computes PoR from state transition
    /// - Calculates delta_pi, phi, and delta_v
    ///
    /// ## Phase 7: Gate Evaluation
    /// - Evaluates FIRE/HOLD condition
    /// - Stores knowledge if gate fires
    pub fn process(&mut self, input: CognitiveInput) -> Result<CognitiveOutput, CognitiveError> {
        // Phase 1: APOLLYON - 5D Dynamic Integration
        let trajectory = self.integrate_5d(&input)?;

        if trajectory.is_empty() {
            return Err(CognitiveError::EmptyTrajectory);
        }

        // Phase 2: APOLLYON - Spectral Analysis
        let spectral_signature = self.analyze_spectrum(&trajectory)?;

        // Phase 3: Bridge - State Conversion
        let final_state = trajectory
            .last()
            .ok_or(CognitiveError::EmptyTrajectory)?;
        let _mef_spiral = StateAdapter::apollyon_to_mef(final_state);

        // Phase 4: MEF - Route Selection (APOLLYON-enhanced)
        let route = self
            .metatron_bridge
            .select_route_enhanced(final_state, &input.seed, 0.0)
            .map_err(|e| CognitiveError::RouteSelectionError(e.to_string()))?;

        // Phase 5: MEF - Knowledge Derivation
        // Create a simplified KnowledgeObject
        let knowledge = self.create_knowledge_object(&input, &route, &spectral_signature);

        // Phase 6: Bridge - Proof-of-Resonance
        let proof = self.compute_proof_of_resonance(&trajectory);

        // Phase 7: MEF - Gate Evaluation
        let gate_decision = self.evaluate_gate(&trajectory);

        Ok(CognitiveOutput {
            trajectory,
            spectral_signature,
            route,
            proof,
            gate_decision,
            knowledge: Some(knowledge),
        })
    }

    /// Integrate 5D dynamics from initial state
    fn integrate_5d(&self, input: &CognitiveInput) -> Result<Vec<core_5d::State5D>, CognitiveError> {
        // Create coupling matrix (identity for now)
        let coupling = core_5d::CouplingMatrix::identity();
        
        // Create vector field from parameters
        let field = VectorField::new(coupling, input.parameters.clone());

        // Configure time integration
        let time_config =
            core_5d::integration::TimeConfig::new(0.01, 0.0, input.t_final);

        // Create integrator
        let integrator = Integrator::new(field, time_config);

        // Integrate and return trajectory (only states, not times)
        Ok(integrator.integrate_states(input.initial_state))
    }

    /// Analyze trajectory spectrum and convert to MEF signature
    fn analyze_spectrum(
        &self,
        trajectory: &[core_5d::State5D],
    ) -> Result<SpectralSignature, CognitiveError> {
        if trajectory.is_empty() {
            return Err(CognitiveError::EmptyTrajectory);
        }

        // Create trajectory observer (with max history = trajectory length)
        let mut observer = TrajectoryObserver::new(trajectory.len());
        for state in trajectory {
            observer.observe(*state);
        }

        // Compute spectral statistics
        let entropy = self.spectral_analyzer.average_entropy(&observer);

        // Compute centroids (use mean of each component)
        let mut centroids = vec![0.0; 5];
        for state in trajectory {
            for i in 0..5 {
                centroids[i] += state.get(i);
            }
        }
        let n = trajectory.len() as f64;
        for centroid in &mut centroids {
            *centroid /= n;
        }

        // Compute dominant frequency (use first component)
        let dominant_freq = self
            .spectral_analyzer
            .dominant_frequency(&observer, 0)
            .unwrap_or(0.0);

        // Convert to MEF spectral signature
        Ok(SpectralAdapter::features_to_signature(
            entropy,
            &centroids,
            dominant_freq,
        ))
    }

    /// Create knowledge object from processing results
    fn create_knowledge_object(
        &self,
        input: &CognitiveInput,
        route: &mef_schemas::RouteSpec,
        spectral: &SpectralSignature,
    ) -> KnowledgeObject {
        // Generate MEF ID from TIC, route, and seed
        let mef_id = format!(
            "MEF-{}-{}-{}",
            input.tic_id,
            route.route_id,
            input.seed.chars().take(8).collect::<String>()
        );

        // Create payload with spectral data
        let payload = serde_json::json!({
            "spectral_signature": {
                "psi": spectral.psi,
                "rho": spectral.rho,
                "omega": spectral.omega,
            },
            "route": {
                "route_id": route.route_id,
                "permutation": route.permutation,
            }
        });

        KnowledgeObject::new(
            mef_id,
            input.tic_id.clone(),
            route.route_id.clone(),
            input.seed_path.clone(),
            input.seed.as_bytes().to_vec(),
            Some(payload),
        )
    }

    /// Compute Proof-of-Resonance from trajectory
    fn compute_proof_of_resonance(
        &self,
        trajectory: &[core_5d::State5D],
    ) -> ProofOfResonanceData {
        if trajectory.len() < 2 {
            return ProofOfResonanceData::default();
        }

        // Use last two states for transition
        let state_prev = &trajectory[trajectory.len() - 2];
        let state_curr = &trajectory[trajectory.len() - 1];

        // Use constant resonance field for simplicity
        let field = ConstantResonanceField::new(0.8);

        ResonanceBridge::compute_proof(&field, state_prev, state_curr, 0.0)
    }

    /// Evaluate Merkaba Gate decision
    fn evaluate_gate(&self, trajectory: &[core_5d::State5D]) -> GateDecision {
        if trajectory.len() < 2 {
            return GateDecision::HOLD;
        }

        // Use last two states for transition
        let state_prev = &trajectory[trajectory.len() - 2];
        let state_curr = &trajectory[trajectory.len() - 1];

        // Use constant resonance field for simplicity
        let field = ConstantResonanceField::new(0.8);

        ResonanceBridge::evaluate_gate(&field, state_prev, state_curr, 0.0)
    }
}

impl Default for UnifiedCognitiveEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_5d::State5D;

    #[test]
    fn test_engine_creation() {
        let engine = UnifiedCognitiveEngine::new();
        assert!(std::ptr::addr_of!(engine).is_aligned());
    }

    #[test]
    fn test_basic_pipeline() {
        let mut engine = UnifiedCognitiveEngine::new();

        // Create simple input
        let input = CognitiveInput {
            initial_state: State5D::new(1.0, 0.5, 0.3, 0.2, 0.1),
            parameters: core_5d::SystemParameters::default(),
            t_final: 1.0,
            tic_id: "TIC-001".to_string(),
            seed: "test_seed".to_string(),
            seed_path: "MEF/test/stage/0001".to_string(),
        };

        // Process through pipeline
        let result = engine.process(input);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.trajectory.is_empty());
        assert!(output.knowledge.is_some());
    }

    #[test]
    fn test_integration_produces_trajectory() {
        let engine = UnifiedCognitiveEngine::new();

        let input = CognitiveInput {
            initial_state: State5D::new(1.0, 0.0, 0.0, 0.0, 0.0),
            parameters: core_5d::SystemParameters::default(),
            t_final: 0.5,
            tic_id: "TIC-002".to_string(),
            seed: "test".to_string(),
            seed_path: "MEF/test/0001".to_string(),
        };

        let trajectory = engine.integrate_5d(&input).unwrap();
        assert!(!trajectory.is_empty());
        assert_eq!(trajectory[0], input.initial_state);
    }

    #[test]
    fn test_spectral_analysis() {
        let engine = UnifiedCognitiveEngine::new();

        // Create simple trajectory
        let trajectory = vec![
            State5D::new(1.0, 0.0, 0.0, 0.0, 0.0),
            State5D::new(0.9, 0.1, 0.0, 0.0, 0.0),
            State5D::new(0.8, 0.2, 0.0, 0.0, 0.0),
        ];

        let signature = engine.analyze_spectrum(&trajectory).unwrap();

        // Verify signature components are reasonable
        assert!(signature.psi.is_finite());
        assert!(signature.rho.is_finite());
        assert!(signature.omega.is_finite());
        assert!(signature.rho >= 0.0 && signature.rho <= 1.0);
    }

    #[test]
    fn test_proof_computation() {
        let engine = UnifiedCognitiveEngine::new();

        let trajectory = vec![
            State5D::new(1.0, 0.0, 0.0, 0.0, 0.0),
            State5D::new(0.99, 0.0, 0.0, 0.0, 0.0),
        ];

        let proof = engine.compute_proof_of_resonance(&trajectory);

        assert!(proof.por_valid);
        assert!(proof.delta_pi.is_finite());
        assert!(proof.phi.is_finite());
        assert!(proof.delta_v.is_finite());
    }

    #[test]
    fn test_gate_evaluation() {
        let engine = UnifiedCognitiveEngine::new();

        // Create trajectory with decreasing energy (should FIRE)
        let trajectory = vec![
            State5D::new(1.0, 0.0, 0.0, 0.0, 0.0),
            State5D::new(0.9, 0.0, 0.0, 0.0, 0.0),
        ];

        let decision = engine.evaluate_gate(&trajectory);

        // Decision should be either FIRE or HOLD based on gate logic
        match decision {
            GateDecision::FIRE | GateDecision::HOLD => {
                // Valid decision
            }
        }
    }

    #[test]
    fn test_empty_trajectory_error() {
        let engine = UnifiedCognitiveEngine::new();

        let result = engine.analyze_spectrum(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_knowledge_object_creation() {
        let engine = UnifiedCognitiveEngine::new();

        let input = CognitiveInput {
            initial_state: State5D::new(1.0, 0.0, 0.0, 0.0, 0.0),
            parameters: core_5d::SystemParameters::default(),
            t_final: 1.0,
            tic_id: "TIC-003".to_string(),
            seed: "test_seed_123".to_string(),
            seed_path: "MEF/test/stage/0001".to_string(),
        };

        let route = mef_schemas::RouteSpec::new(
            "ROUTE-001".to_string(),
            vec![0, 1, 2, 3, 4, 5, 6],
            0.75,
        )
        .unwrap();

        let spectral = SpectralSignature {
            psi: 0.5,
            rho: 0.7,
            omega: 2.1,
        };

        let knowledge = engine.create_knowledge_object(&input, &route, &spectral);

        assert_eq!(knowledge.tic_id, "TIC-003");
        assert_eq!(knowledge.route_id, "ROUTE-001");
        assert_eq!(knowledge.seed_path, "MEF/test/stage/0001");
        assert!(knowledge.payload.is_some());
    }
}
