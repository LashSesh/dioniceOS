# Quick Start Guide - Unified 5D Cube Overlay

## Installation

The overlay is already part of the workspace. No additional installation needed.

```bash
cd overlay/unified_5d_cube
cargo test    # Run tests
cargo build   # Build library
```

## Run Examples

```bash
# Basic usage
cargo run --example simple

# Metrics collection and export
cargo run --example metrics

# Determinism verification
cargo run --example replay
```

## Basic Usage

```rust
use unified_5d_cube::{InterlockConfig, InterlockAdapter, tick_5d_cube};
use core_5d::State5D;

fn main() {
    // 1. Create configuration
    let config = InterlockConfig::default();
    
    // 2. Create interlock adapter
    let mut adapter = InterlockAdapter::new(config);
    
    // 3. Initial state
    let state = State5D::from_array([1.0, 0.5, 0.3, 0.7, 0.4]);
    
    // 4. Execute tick
    let result = tick_5d_cube(&mut adapter, &state, None, 0.0, 0);
    
    // 5. Access results
    println!("Gate: {:?}", result.gate_decision);
    println!("ΔF: {:.4}", result.metrics.delta_f);
}
```

## Key Features

- **Non-invasive**: Uses only public APIs
- **Shadow Mode**: Default (no side effects)
- **Deterministic**: Fixed seed = reproducible results
- **Metrics**: CSV/JSON export
- **Tested**: 9 unit tests passing

## Documentation

- `README.md` - Overview
- `interlock_map.md` - Architecture details
- `IMPLEMENTATION_SUMMARY.md` - Complete deliverables

## Configuration

```rust
let config = InterlockConfig {
    seed: 42,                      // Deterministic seed
    gate_phi_threshold: 0.5,       // Min alignment
    gate_delta_pi_max: 0.1,        // Max path invariance
    enable_logging: true,
    shadow_mode: true,             // Start in shadow
};
```

## Next Steps

1. Read `interlock_map.md` for architecture details
2. Run examples to see functionality
3. Review tests in `src/` modules
4. Check `IMPLEMENTATION_SUMMARY.md` for complete overview

