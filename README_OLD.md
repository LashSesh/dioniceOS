# dioniceOS - Geometric-Cognitive Computing Platform

**Integration of APOLLYON-5D and Infinity-Ledger (MEF-Core)**

This repository contains the holistic integration of two production-ready systems into a unified Geometric-Cognitive Computing Platform:

1. **APOLLYON-5D**: A 5-dimensional geometric-cognitive mathematics engine
2. **Infinity-Ledger (MEF-Core)**: A proof-carrying vector ledger engine

---

## 📚 Documentation

### Integration Plan
- **[INTEGRATION_PLAN.md](./INTEGRATION_PLAN.md)** - Complete step-by-step implementation plan for integrating both systems
- **[apollyon_mef.md](./apollyon_mef.md)** - Detailed analysis of system compatibility and integration potential

### System Documentation
- **[apollyon_5d/README.md](./apollyon_5d/README.md)** - APOLLYON-5D documentation
- **[infinity-ledger/README.md](./infinity-ledger/README.md)** - Infinity-Ledger (MEF-Core) documentation

---

## 🏗️ Repository Structure

```
dioniceOS/
├── INTEGRATION_PLAN.md          # Master integration plan (READ THIS FIRST!)
├── apollyon_mef.md              # Detailed analysis document
├── README.md                    # This file
├── Cargo.toml                   # Root workspace (bridge only)
│
├── apollyon_5d/                 # APOLLYON-5D System (Independent workspace)
│   ├── Cargo.toml               # APOLLYON workspace config
│   ├── core/                    # 5D dynamical systems framework
│   ├── metatron/                # Geometric cognition engine
│   └── bridge/                  # Adaptive integration layer
│
├── infinity-ledger/             # Infinity-Ledger System (Independent workspace)
│   ├── Cargo.toml               # MEF workspace config
│   ├── mef-core/                # Core MEF pipeline
│   ├── mef-spiral/              # Spiral snapshot system
│   ├── mef-ledger/              # Hash-chained ledger
│   ├── mef-knowledge/           # Knowledge derivation
│   ├── mef-memory/              # Vector memory
│   ├── mef-router/              # Metatron S7 routing
│   ├── mef-schemas/             # Type system
│   └── [other MEF modules]/
│
└── apollyon-mef-bridge/         # Integration Bridge (NEW!)
    ├── Cargo.toml
    ├── src/
    │   ├── adapters/            # Bidirectional type converters
    │   │   ├── state_adapter.rs       # ✅ 5D ⟷ Spiral (COMPLETE)
    │   │   ├── spectral_adapter.rs    # ✅ Features ⟷ Signature (COMPLETE)
    │   │   ├── metatron_adapter.rs    # Cube-13 ⟷ S7 (placeholder)
    │   │   └── resonance_adapter.rs   # Field ⟷ PoR (placeholder)
    │   ├── pipeline/            # Processing pipelines
    │   └── unified/             # Unified cognitive engine
    └── tests/
```

---

## 🎯 Integration Status

### ✅ Completed
- [x] System analysis and compatibility verification
- [x] Comprehensive integration plan (INTEGRATION_PLAN.md)
- [x] Systems extracted and organized
- [x] Bridge crate structure created
- [x] State Adapter: Perfect 5D ⟷ Spiral conversion (with tests)
- [x] Spectral Adapter: Features ⟷ Signature mapping (with tests)

### 🚧 In Progress / Placeholders
- [ ] Metatron Bridge (Cube-13 ⟷ S7 Router)
- [ ] Resonance Bridge (ResonanceField ⟷ PoR)
- [ ] Unified Cognitive Engine
- [ ] Integration tests
- [ ] Example applications

---

## 🔑 Key Integration Insights

### Perfect Mathematical Alignment

Both systems operate in the **same 5D mathematical space**:

| Dimension | APOLLYON-5D | MEF-Core | Meaning |
|-----------|-------------|----------|---------|
| D1 | x (index 0) | coords[0] | Spatial X |
| D2 | y (index 1) | coords[1] | Spatial Y |
| D3 | z (index 2) | coords[2] | Spatial Z |
| D4 | ψ/psi (index 3) | coords[3] | Semantic weight / Resonance |
| D5 | ω/omega (index 4) | coords[4] | Temporal phase / Oscillation |

This **1:1 mapping** enables lossless bidirectional conversion with roundtrip error < 1e-10.

### Complementary Capabilities

- **APOLLYON-5D**: Dynamic computation (ephemeral state evolution)
- **MEF-Core**: Persistent storage (immutable proof-carrying ledger)
- **Integration**: Computation + Verification + Storage

---

## 🚀 Building the Systems

### Prerequisites

```bash
# Install Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build APOLLYON-5D

```bash
cd apollyon_5d
cargo build --release
cargo test --release
```

**Expected**: 109 tests passing ✅

### Build Infinity-Ledger

```bash
cd infinity-ledger
cargo build --release --workspace
cargo test --workspace
```

### Build Integration Bridge

**Note**: The bridge crate currently has placeholder implementations. To build a working version, follow the steps in `INTEGRATION_PLAN.md`.

```bash
# Bridge crate references both systems via path dependencies
# Full integration requires completing the implementation
cd apollyon-mef-bridge
# cargo build  # Not yet fully functional
```

---

## 📖 Understanding the Integration

### The Integration Plan (INTEGRATION_PLAN.md)

The `INTEGRATION_PLAN.md` document provides a **complete, deterministic, step-by-step guide** for implementing the full integration. It includes:

1. **System Analysis**: Deep dive into both systems' architectures
2. **Integration Architecture**: Unified layer model
3. **Implementation Steps**: 10 detailed steps with code examples
4. **Use Cases**: 4 real-world application scenarios
5. **Performance Targets**: Benchmarks and optimization goals
6. **Success Criteria**: Clear validation checkpoints

### Current Implementation

The repository currently contains:

1. **Complete State Adapter** (`apollyon-mef-bridge/src/adapters/state_adapter.rs`)
   - Bidirectional conversion between State5D and Vec<f64> coordinates
   - Perfect roundtrip validation (error < 1e-10)
   - Comprehensive test coverage

2. **Complete Spectral Adapter** (`apollyon-mef-bridge/src/adapters/spectral_adapter.rs`)
   - Converts spectral analysis features to MEF signatures
   - Maps entropy → resonance (inverse relationship)
   - Maps centroids → phase alignment
   - Maps frequency → oscillation

3. **Placeholder Structures**
   - Metatron Bridge
   - Resonance Bridge
   - Unified Cognitive Engine
   - Pipeline components

---

## 🎓 Use Cases (From Integration Plan)

### 1. Verifiable AI Reasoning
Encode queries in 5D space → Integrate dynamics → Generate proofs → Store in ledger

### 2. Geometric Knowledge Graphs
Concepts as 5D nodes → Relationships as couplings → Vector search in 8D space

### 3. Temporal Concept Evolution
Track concept drift over time with verified transitions and cryptographic proofs

### 4. Self-Improving Systems
System analyzes performance → Generates proposals → Verifies improvements → Stores proofs

---

## 🔬 Technical Specifications

### APOLLYON-5D

- **Framework**: 5D dynamical systems with Heun's method (RK2) integration
- **Cognition**: Metatron-R 13-node geometric structure with QLogic/QDASH
- **Tests**: 109/109 passing (39 Core + 32 Metatron + 38 Bridge)
- **Language**: Rust 2021 edition

### Infinity-Ledger (MEF-Core)

- **Engine**: Proof-carrying vector ledger with SHA-256 hash chaining
- **Storage**: TIC snapshots with S3/MinIO support
- **Routing**: Metatron S7 (7! = 5040 permutations)
- **Vector Memory**: HNSW/FAISS indexing in 8D space
- **Language**: Rust 2021 edition

### Integration Bridge

- **Adapters**: Type-safe bidirectional converters
- **Pipeline**: Sequential and parallel processing modes
- **Engine**: Unified orchestration of both systems
- **Target Latency**: < 20ms end-to-end pipeline

---

## 📝 Next Steps

To complete the integration, follow the steps in `INTEGRATION_PLAN.md`:

1. **Step 3-6**: Implement remaining adapters (Metatron, Resonance)
2. **Step 7**: Complete Unified Cognitive Engine
3. **Step 8**: Add comprehensive integration tests
4. **Step 9**: Create example applications
5. **Step 10**: Document and validate

Each step includes:
- Detailed implementation code
- Test cases
- Success criteria
- Verification commands

---

## 🤝 Contributing

This is a research integration project. The integration plan provides a complete roadmap for contributions. Key areas:

1. Completing the adapter implementations
2. Building the unified cognitive engine
3. Creating example applications
4. Performance optimization
5. Documentation improvements

---

## 📄 License

- **APOLLYON-5D**: See `apollyon_5d/` for license information
- **Infinity-Ledger**: MIT License (see `infinity-ledger/LICENSE`)
- **Integration Bridge**: MIT License

---

## 🌟 Project Vision

**"Bridging deterministic mathematics with geometric cognition to create the world's first cryptographically-verifiable geometric-cognitive computing platform."**

This integration combines:
- ✅ Deterministic 5D dynamics (APOLLYON)
- ✅ Immutable proof-carrying storage (MEF)
- ✅ Cryptographic verification (PoR)
- ✅ Vector search in 8D space (HNSW)
- ✅ Temporal provenance (TICs)

---

**For detailed implementation instructions, see [INTEGRATION_PLAN.md](./INTEGRATION_PLAN.md)**

**Last Updated**: October 2025  
**Version**: 0.1.0 (Integration Phase)  
**Status**: Foundation Complete, Full Implementation In Progress
