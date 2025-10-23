//! Interlock adapters connecting APOLLYON, Trichter, and MEF via public APIs only.
//!
//! This module provides the glue layer that connects:
//! - STATE_IN: APOLLYON State5D
//! - FIELD_IO: Trichter ∇Φ, Hyperbion/HDAG
//! - GATE: MEF Proof-of-Resonance / Merkaba
//! - CONDENSE: Trichter Coagula/Tick
//! - EVENT_OUT: MEF Ledger/TIC

use core_5d::State5D as ApollonState5D;
use apollyon_mef_bridge::trichter::{State5D as TrichterState5D, HDAGField, Hyperbion, FunnelGraph};
use mef_schemas::GateDecision;
use serde::{Deserialize, Serialize};

/// Simple Proof-of-Resonance data for the 5D Cube
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleProofOfResonance {
    /// Path invariance (Wasserstein-2 distance)
    pub delta_pi: f64,
    
    /// Alignment (cosine similarity)
    pub phi: f64,
    
    /// Lyapunov delta (energy change)
    pub delta_v: f64,
    
    /// Overall validity
    pub por_valid: bool,
}

/// Configuration for the interlock system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterlockConfig {
    /// Deterministic seed for reproducibility
    pub seed: u64,
    
    /// Gate thresholds
    pub gate_phi_threshold: f64,
    pub gate_delta_pi_max: f64,
    
    /// Feature flags
    pub enable_logging: bool,
    pub shadow_mode: bool,
}

impl Default for InterlockConfig {
    fn default() -> Self {
        Self {
            seed: 42,
            gate_phi_threshold: 0.5,
            gate_delta_pi_max: 0.1,
            enable_logging: true,
            shadow_mode: true, // Start in shadow mode by default
        }
    }
}

/// Adapter connecting all components via public APIs
pub struct InterlockAdapter {
    config: InterlockConfig,
    
    // Trichter components
    hdag: HDAGField,
    hyperbion: Hyperbion,
    funnel: FunnelGraph,
}

impl InterlockAdapter {
    /// Create new interlock adapter with configuration
    pub fn new(config: InterlockConfig) -> Self {
        let hyperbion = Hyperbion::new();
        let hdag = HDAGField::new();
        let funnel = FunnelGraph::new();
        
        Self {
            config,
            hdag,
            hyperbion,
            funnel,
        }
    }
    
    /// STATE_IN: Convert APOLLYON State5D to Trichter State5D
    pub fn apollyon_to_trichter(&self, apollon: &ApollonState5D) -> TrichterState5D {
        // Use public API conversion (State5D has to_array())
        let arr = apollon.to_array();
        TrichterState5D::new(arr[0], arr[1], arr[2], arr[3], arr[4])
    }
    
    /// FIELD_IO: Compute guidance field using Trichter Hyperbion + HDAG
    /// This is a simplified version that computes a basic guidance vector
    pub fn compute_guidance(&mut self, state: &TrichterState5D, _t: f64) -> [f64; 5] {
        // Simplified guidance computation based on gradients toward origin
        // In full implementation, would use actual HDAG relaxation and Hyperbion fields
        let arr = state.as_array();
        let norm = state.norm();
        
        if norm < 1e-10 {
            return [0.0; 5];
        }
        
        // Simple gradient descent toward equilibrium
        let scale = -0.1; // Small step
        [
            arr[0] * scale,
            arr[1] * scale,
            arr[2] * scale,
            arr[3] * scale,
            arr[4] * scale,
        ]
    }
    
    /// GATE: Evaluate Merkaba gate using simplified Proof-of-Resonance
    pub fn evaluate_gate(
        &self,
        state_prev: &TrichterState5D,
        state_curr: &TrichterState5D,
        _delta_t: f64,
    ) -> (GateDecision, SimpleProofOfResonance) {
        // Compute path invariance
        let delta_pi = self.compute_path_invariance(state_prev, state_curr);
        
        // Compute alignment (simplified - using state norm as proxy)
        let phi = self.compute_alignment(state_prev, state_curr);
        
        // Compute Lyapunov delta
        let delta_v = self.compute_lyapunov_delta(state_prev, state_curr);
        
        // Create PoR
        let por_valid = delta_pi <= self.config.gate_delta_pi_max 
                        && phi >= self.config.gate_phi_threshold;
        
        let proof = SimpleProofOfResonance {
            delta_pi,
            phi,
            delta_v,
            por_valid,
        };
        
        // Evaluate gate decision
        let decision = if proof.por_valid && proof.delta_v < 0.0 {
            GateDecision::FIRE
        } else {
            GateDecision::HOLD
        };
        
        (decision, proof)
    }
    
    /// CONDENSE: Apply Trichter funnel operations (coagula)
    pub fn condense(&mut self, state: &TrichterState5D, guidance: &[f64; 5]) -> TrichterState5D {
        // This is a simplified coagulation step
        // Apply guidance as condensation force
        let arr = state.as_array();
        
        TrichterState5D::new(
            arr[0] + guidance[0],
            arr[1] + guidance[1],
            arr[2] + guidance[2],
            arr[3] + guidance[3],
            arr[4] + guidance[4],
        )
    }
    
    /// EVENT_OUT: Prepare commit data for MEF Ledger
    pub fn prepare_commit(
        &self,
        state: &TrichterState5D,
        proof: &SimpleProofOfResonance,
    ) -> CommitData {
        use sha2::{Sha256, Digest};
        
        // Create deterministic hash
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", state.as_array()));
        hasher.update(format!("{:.10}", proof.phi));
        hasher.update(format!("{}", self.config.seed));
        let hash = format!("{:x}", hasher.finalize());
        
        CommitData {
            state: *state,
            proof: proof.clone(),
            commit_hash: hash,
            timestamp: chrono::Utc::now(),
        }
    }
    
    // Helper methods
    
    fn compute_path_invariance(&self, prev: &TrichterState5D, curr: &TrichterState5D) -> f64 {
        let p = prev.as_array();
        let c = curr.as_array();
        let mut sum = 0.0;
        for i in 0..5 {
            let diff = c[i] - p[i];
            sum += diff * diff;
        }
        sum.sqrt()
    }
    
    fn compute_alignment(&self, prev: &TrichterState5D, curr: &TrichterState5D) -> f64 {
        // Simplified alignment: cosine similarity
        let p = prev.as_array();
        let c = curr.as_array();
        
        let mut dot = 0.0;
        let mut norm_prev = 0.0;
        let mut norm_curr = 0.0;
        
        for i in 0..5 {
            dot += p[i] * c[i];
            norm_prev += p[i] * p[i];
            norm_curr += c[i] * c[i];
        }
        
        if norm_prev == 0.0 || norm_curr == 0.0 {
            return 0.0;
        }
        
        dot / (norm_prev.sqrt() * norm_curr.sqrt())
    }
    
    fn compute_lyapunov_delta(&self, prev: &TrichterState5D, curr: &TrichterState5D) -> f64 {
        let v_prev = prev.norm();
        let v_curr = curr.norm();
        v_curr - v_prev
    }
}

/// Data structure for MEF commits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitData {
    pub state: TrichterState5D,
    pub proof: SimpleProofOfResonance,
    pub commit_hash: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_interlock_creation() {
        let config = InterlockConfig::default();
        let _adapter = InterlockAdapter::new(config);
    }
    
    #[test]
    fn test_state_conversion() {
        let config = InterlockConfig::default();
        let adapter = InterlockAdapter::new(config);
        
        let apollon = ApollonState5D::from_array([1.0, 2.0, 3.0, 0.5, 0.7]);
        let trichter = adapter.apollyon_to_trichter(&apollon);
        
        assert_eq!(trichter.as_array()[0], 1.0);
        assert_eq!(trichter.as_array()[4], 0.7);
    }
}
