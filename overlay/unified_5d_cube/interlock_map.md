# Unified 5D Cube - Interlock Map

## Overview

The Unified 5D Cube overlay integrates APOLLYON-5D, 4D-Trichter, and MEF-Core through **public APIs only**. This document describes the interlock points and data flow.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Unified 5D Cube Overlay                   │
│                        (Shadow Mode)                          │
└─────────────────────────────────────────────────────────────┘
                              │
    ┌────────────────────────┼────────────────────────┐
    │                        │                        │
    ▼                        ▼                        ▼
┌─────────┐            ┌──────────┐           ┌──────────┐
│APOLLYON │            │ Trichter │           │   MEF    │
│  5D     │            │ 4D-Funnel│           │  Core    │
└─────────┘            └──────────┘           └──────────┘
```

## Interlock Points

### 1. STATE_IN: APOLLYON → Trichter

**Source**: `core_5d::State5D` (APOLLYON)  
**Target**: `apollyon_mef_bridge::trichter::State5D`  
**Method**: `InterlockAdapter::apollyon_to_trichter()`

```rust
// Public API used:
apollon.to_array() -> [f64; 5]
TrichterState5D::new(x, y, z, psi, omega)
```

**Data Flow**:
- Extract 5D components from APOLLYON state
- Create Trichter state with same coordinates
- Preserves: x, y, z (spatial), ψ (semantic), ω (temporal)

---

### 2. FIELD_IO: Trichter ∇Φ

**Components**: `Hyperbion`, `HDAGField`  
**Method**: `InterlockAdapter::compute_guidance()`

```rust
// Public APIs used:
Hyperbion::new()
HDAGField::new()
```

**Computation**:
1. Hyperbion fields H(x,t) = α·Φ + β·μ (simplified in current version)
2. HDAG relaxation with Hyperbion fields (planned)
3. Gradient computation ∇Φ for guidance

**Output**: `[f64; 5]` guidance vector

---

### 3. GATE: MEF Proof-of-Resonance

**Source**: Previous + Current Trichter states  
**Target**: `mef_schemas::GateDecision` + `SimpleProofOfResonance`  
**Method**: `InterlockAdapter::evaluate_gate()`

**Metrics Computed**:
- **ΔPI**: Path invariance (Wasserstein-2 distance)
  ```
  ΔPI = ||state_curr - state_prev||₂
  ```
  
- **Φ**: Alignment (cosine similarity)
  ```
  Φ = <state_curr, state_prev> / (||state_curr|| · ||state_prev||)
  ```
  
- **ΔV**: Lyapunov delta (energy change)
  ```
  ΔV = ||state_curr|| - ||state_prev||
  ```

**Gate Decision**:
```
FIRE ⟺ (PoR_valid) ∧ (ΔPI ≤ ε) ∧ (Φ ≥ φ) ∧ (ΔV < 0)
```

Thresholds:
- ε (delta_pi_max) = 0.1
- φ (phi_threshold) = 0.5

---

### 4. CONDENSE: Trichter Coagula

**Method**: `InterlockAdapter::condense()`

**Operation**:
```rust
state_condensed = state + guidance * α
```
where α is the condensation factor (currently 1.0)

**Purpose**: Apply guidance field to evolve state

---

### 5. EVENT_OUT: MEF Commit

**Target**: `CommitData` for MEF Ledger/TIC  
**Method**: `InterlockAdapter::prepare_commit()`

**Commit Data**:
```rust
{
    state: TrichterState5D,
    proof: SimpleProofOfResonance,
    commit_hash: SHA256(state + proof.phi + seed),
    timestamp: UTC
}
```

**Determinism**: Uses fixed seed (default: 42) for reproducible hashes

---

## Tick Pipeline

The main execution flow through `tick_5d_cube()`:

```
┌──────────────────────────────────────────────────────┐
│ Phase 1: STATE_IN                                     │
│ APOLLYON State5D → Trichter State5D                   │
└──────────────────────────────────────────────────────┘
                      │
┌──────────────────────────────────────────────────────┐
│ Phase 2: Solve/Relax                                  │
│ (Conceptual - state already solved from APOLLYON)     │
└──────────────────────────────────────────────────────┘
                      │
┌──────────────────────────────────────────────────────┐
│ Phase 3: FIELD_IO                                     │
│ Compute guidance field ∇Φ via Trichter                │
└──────────────────────────────────────────────────────┘
                      │
┌──────────────────────────────────────────────────────┐
│ Phase 4: GATE                                         │
│ Evaluate MEF Proof-of-Resonance                       │
│ Decision: FIRE or HOLD                                │
└──────────────────────────────────────────────────────┘
                      │
┌──────────────────────────────────────────────────────┐
│ Phase 5: CONDENSE                                     │
│ Apply Trichter coagulation                            │
└──────────────────────────────────────────────────────┘
                      │
┌──────────────────────────────────────────────────────┐
│ Phase 6: Collapse → Commit (if FIRE)                  │
│ Prepare commit data for MEF Ledger                    │
└──────────────────────────────────────────────────────┘
```

---

## Metrics Collection

Each tick produces the following metrics (logged to CSV/JSON):

| Metric    | Description                     | Source                |
|-----------|---------------------------------|-----------------------|
| BI        | Betti number approx.            | Guidance magnitude    |
| ΔF        | Energy delta (Lyapunov)         | Gate PoR              |
| W2_step   | Wasserstein-2 distance          | Gate PoR              |
| λ_gap     | Spectral gap                    | State eigenvalues     |
| S_mand    | Mandorla score (coherence)      | State variance        |
| Duty/PoR  | PoR validity (1.0 or 0.0)       | Gate decision         |

---

## Shadow Mode & Activation

### Shadow Mode (Default)

- **No side effects**: Commits are prepared but not executed
- **Logging only**: All metrics collected, gates evaluated
- **Purpose**: Validation before production activation

### Activation Criteria

System activates from Shadow → Active when **all** conditions met over **3 consecutive windows**:

1. **ΔF ≤ 0** (energy decreasing)
2. **W2_step ↓** (states converging)
3. **Gate stable** (≥80% FIRE rate, no flicker)
4. **Coherence high** (S_mand ≥ 0.7)

### Auto-Rollback

If active system shows instability:
- ΔF increases
- Coherence drops
- Gate becomes unstable

→ **Automatic rollback** to Shadow mode

---

## Determinism & Reproducibility

### Seeds

- Default seed: **42**
- Configurable via `InterlockConfig::seed`

### Commit Hashes

```
hash = SHA256(
    state_array +
    proof.phi +
    seed
)
```

→ **Deterministic** for same inputs and seed

### Replay

Given same:
- Initial state
- Seed
- Tick sequence

→ Produces **identical** commit hashes (MEF Ledger compatible)

---

## Feature Flags

### `activate` (default: OFF)

```toml
# Enable in Cargo.toml:
[features]
activate = []
```

**Without flag**: Always runs in shadow mode  
**With flag**: Allows shadow → active transition

**Check at runtime**:
```rust
unified_5d_cube::is_activated()
```

---

## Public APIs Used

### From APOLLYON (core_5d)
- `State5D::from_array()` - constructor
- `State5D::to_array()` - extract components

### From Trichter (apollyon_mef_bridge::trichter)
- `State5D::new()` - constructor
- `State5D::as_array()` - extract components
- `State5D::norm()` - L2 norm
- `Hyperbion::new()` - create Hyperbion
- `HDAGField::new()` - create HDAG
- `FunnelGraph::new()` - create funnel

### From MEF (mef_schemas)
- `GateDecision::FIRE` - gate fires
- `GateDecision::HOLD` - gate holds

---

## Non-Invasive Design

**No changes to base systems**:
- APOLLYON: unchanged
- Trichter: unchanged  
- MEF: unchanged

**All integration** via:
- Public constructors
- Public getters
- Public data structures

→ **Pure overlay** pattern

---

## Future Extensions

1. **Full HDAG Relaxation**: Implement actual Hyperbion field computation and HDAG gradient
2. **Funnel Operations**: Integrate split/merge/prune from FunnelGraph
3. **MEF Ledger Integration**: Actual TIC writes instead of prepared commits
4. **8D Vector Construction**: Full MEF knowledge derivation pipeline
5. **Metatron Router**: Route selection via S7 permutations

---

Last Updated: 2025-10-23  
Version: 0.1.0
