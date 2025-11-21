# dioniceOS Architecture

This document provides a detailed architectural overview of the dioniceOS platform.

## System Overview

dioniceOS integrates three mathematical frameworks into a unified geometric-cognitive computing platform:

```
                    ┌─────────────────────────────────┐
                    │     4D-TRICHTER (Gabriel)       │
                    │   Morphodynamic Pattern System  │
                    │                                 │
                    │  ┌─────────┐ ┌─────────┐       │
                    │  │ Funnel  │ │Hyperbion│       │
                    │  └────┬────┘ └────┬────┘       │
                    │       │           │            │
                    │       └─────┬─────┘            │
                    │             │                  │
                    │        ┌────┴────┐             │
                    │        │  HDAG   │             │
                    │        │  Field  │             │
                    │        └────┬────┘             │
                    └─────────────┼─────────────────┘
                                  │
            ┌─────────────────────┼─────────────────────┐
            │                     │                     │
   ┌────────▼────────┐   ┌───────┴───────┐   ┌────────▼────────┐
   │  APOLLYON-5D    │   │  Integration  │   │ Infinity-Ledger │
   │                 │   │    Bridge     │   │   (MEF-Core)    │
   │ - 5D Dynamics   │   │               │   │                 │
   │ - Metatron Cube │   │ - Adapters    │   │ - PoR Proofs    │
   │ - Spectral      │   │ - Pipeline    │   │ - Ledger        │
   │ - QLogic        │   │ - Engine      │   │ - S7 Routing    │
   └─────────────────┘   └───────────────┘   └─────────────────┘
```

## Coordinate System

All systems operate in a unified 5-dimensional coordinate space:

| Dimension | Symbol | Description |
|-----------|--------|-------------|
| D1 | x | Spatial X coordinate |
| D2 | y | Spatial Y coordinate |
| D3 | z | Spatial Z coordinate |
| D4 | psi | Semantic weight / Resonance |
| D5 | omega | Temporal phase / Oscillation |

### State Representations

```rust
// 4D State (used by Funnel)
State4D { x, y, z, psi }

// 5D State (used by HDAG, APOLLYON)
State5D { x, y, z, psi, omega }
```

### Lift and Projection

```
lift: R^4 -> R^5
lift((x, y, z, psi), omega) = (x, y, z, psi, omega)

proj_4D: R^5 -> R^4
proj_4D(x, y, z, psi, omega) = (x, y, z, psi)
```

## Component Details

### 4D-Trichter (Gabriel)

The 4D-Trichter is a kinetic funnel compressor with three layers:

#### 1. Funnel Graph
- Directed graph with 5D state vector nodes
- Hebbian-weighted edges with phase locking
- Operations: Split, Merge, Prune

#### 2. Hyperbion Layer
Provides viscoelastic coupling between 4D flow and 5D field:
```
H(x,t) = alpha * Phi(x,t) + beta * mu(x,t)
```

#### 3. HDAG Field
Hyperdimensional Acyclic Resonance Grid:
- Nodes: 5D resonance tensors
- Edges: Phase-gradient transitions
- Natural acyclicity through phase disalignment

### APOLLYON-5D

Geometric-cognitive mathematics engine:

- **Core**: 5D dynamical systems with Heun's method integration
- **Metatron**: Geometric cognition using Metatron Cube geometry
- **Spectral**: Feature extraction and spectral analysis
- **QLogic**: Spectral gap computation and mesh metrics

### Infinity-Ledger (MEF-Core)

Proof-carrying vector ledger:

- **mef-core**: Core MEF pipeline and types
- **mef-spiral**: Snapshot system for proof-of-resonance
- **mef-ledger**: Hash-chained immutable ledger
- **mef-router**: Metatron S7 routing engine
- **mef-memory**: Vector memory subsystem

## Integration Bridge

The bridge provides bidirectional adapters:

| Adapter | Source | Target | Purpose |
|---------|--------|--------|---------|
| State | State5D | MEF coords | Coordinate mapping |
| Spectral | Features | Signature | Feature translation |
| Metatron | QLogic | S7 Router | Route selection |
| Resonance | Field | PoR | Proof generation |

### Unified Cognitive Engine

8-phase processing pipeline:

1. **Integration**: 5D state integration via Heun's method
2. **Spectral**: Feature extraction
3. **Routing**: S7 route selection
4. **Knowledge**: Derivation from features
5. **Resonance**: Field computation
6. **Gate**: Decision evaluation
7. **Commit**: State recording
8. **Output**: Result packaging

## Data Flow

```
Input (State4D[])
       │
       ▼
  ┌────────────┐
  │    Lift    │  4D -> 5D
  └─────┬──────┘
        │
        ▼
  ┌────────────┐
  │  Hyperbion │  Extract (Phi, mu)
  └─────┬──────┘
        │
        ▼
  ┌────────────┐
  │    HDAG    │  Relax + Gradient
  └─────┬──────┘
        │
        ▼
  ┌────────────┐
  │  Project   │  5D -> 4D
  └─────┬──────┘
        │
        ▼
  ┌────────────┐
  │   Funnel   │  Advect with policy
  └─────┬──────┘
        │
        ▼
  ┌────────────┐
  │   Proof    │  Optional hash
  └─────┬──────┘
        │
        ▼
Output (State4D[], proof?)
```

## Policies

| Policy | Learning | Decay | Use Case |
|--------|----------|-------|----------|
| Explore | High (0.5) | Medium (0.05) | Discovery |
| Exploit | Medium (0.2) | Low (0.01) | Optimization |
| Homeostasis | Adaptive | Adaptive | Stability |

## Security Properties

- **Determinism**: Identical inputs produce identical outputs
- **Offline**: No network dependencies
- **Proofs**: SHA-256 cryptographic hashing
- **Acyclicity**: Natural DAG emergence through phase

## File Structure

```
dioniceOS/
├── apollyon_5d/           # APOLLYON-5D system
│   ├── core/              # 5D dynamics
│   ├── metatron/          # Geometric cognition
│   └── bridge/            # Integration layer
│
├── infinity-ledger/       # MEF-Core system
│   ├── mef-core/          # Core pipeline
│   ├── mef-spiral/        # Snapshots
│   ├── mef-ledger/        # Hash chain
│   └── mef-router/        # S7 routing
│
├── apollyon-mef-bridge/   # Integration
│   ├── src/adapters/      # Bidirectional adapters
│   ├── src/trichter/      # 4D-Trichter impl
│   ├── src/unified/       # Cognitive engine
│   └── src/pipeline/      # Processing
│
└── overlay/               # Unified 5D Cube
    └── unified_5d_cube/   # Interlock overlay
```

## References

- [4D_Trichter.pdf](../4D_Trichter.pdf) - Delta-Blueprint specification
- [5D_Cube.pdf](../5D_Cube.pdf) - 5D system specification
