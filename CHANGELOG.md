# Changelog

All notable changes to dioniceOS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-11-21

### Added

#### Core Systems
- **4D-Trichter (Gabriel)**: Complete implementation of the Delta-Blueprint specification
  - 4D Kinetic Funnel Compressor with Hebbian learning
  - Hyperbion Layer for morphodynamic coupling (H(x,t) = alpha*Phi + beta*mu)
  - HDAG Field (5D Hyperdimensional Acyclic Resonance Grid)
  - Deterministic coupling algorithm with proof generation

- **APOLLYON-5D**: 5-dimensional geometric-cognitive mathematics engine
  - 5D dynamical systems framework with Heun's method integration
  - Metatron Cube geometry and QLogic spectral analysis
  - Spectral features extraction and analysis

- **Infinity-Ledger (MEF-Core)**: Proof-carrying vector ledger
  - Hash-chained immutable ledger for state transitions
  - Proof-of-Resonance validation system
  - Metatron S7 routing engine
  - Vector memory subsystem

#### Integration Bridge
- Bidirectional State Adapter (State5D <-> MEF coordinates)
- Spectral Adapter (Features <-> Signature mapping)
- Metatron Bridge (QLogic <-> S7 Router)
- Resonance Bridge (ResonanceField <-> PoR)
- Unified Cognitive Engine with 8-phase processing pipeline

#### Policies
- **Explore**: High learning rate, discovery-focused
- **Exploit**: Consolidation and optimization
- **Homeostasis**: Adaptive density regulation with hysteresis

#### Unified 5D Cube Overlay
- InterlockAdapter connecting all three systems
- 6-phase execution pipeline (tick_5d_cube)
- ShadowController for shadow mode operation
- MetricsCollector for CSV/JSON export

### Technical Specifications

- **Coordinate Space**: Unified 5D (x, y, z, psi, omega)
- **Lift Operation**: 4D -> 5D transformation
- **Projection**: 5D -> 4D with < 1e-10 roundtrip error
- **Determinism**: Same inputs -> identical outputs
- **Cryptographic**: SHA-256 proof hashing

### Test Coverage

```
Total Tests: 200+
- apollyon_5d:         109 tests
- apollyon-mef-bridge:  96 tests
  - State Adapter:       9 tests
  - Spectral Adapter:   12 tests
  - Metatron Bridge:     6 tests
  - Resonance Bridge:    7 tests
  - Unified Engine:      9 tests
  - 4D-Trichter:        41 tests
- infinity-ledger:     All MEF tests
```

### Dependencies

- Rust 1.70+ (Edition 2021)
- ndarray, nalgebra (linear algebra)
- serde, serde_json (serialization)
- sha2, uuid (cryptography)
- tokio (async runtime)
- tracing (logging)

---

## [0.1.0] - 2025-10-01

### Added
- Initial repository structure
- APOLLYON-5D foundation
- Infinity-Ledger (MEF-Core) foundation
- Basic integration planning

---

[1.0.0]: https://github.com/LashSesh/dioniceOS/releases/tag/v1.0.0
[0.1.0]: https://github.com/LashSesh/dioniceOS/releases/tag/v0.1.0
