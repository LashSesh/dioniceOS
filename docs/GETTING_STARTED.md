# Getting Started with dioniceOS

This guide will help you get up and running with dioniceOS quickly.

## Prerequisites

- Rust 1.70 or later
- Cargo (included with Rust)

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Quick Start

### 1. Clone and Build

```bash
git clone https://github.com/LashSesh/dioniceOS.git
cd dioniceOS

# Build all systems
cargo build --release --workspace
```

### 2. Run Tests

```bash
# Test everything
cargo test --workspace

# Test specific components
cargo test -p apollyon_5d        # APOLLYON-5D
cargo test -p apollyon-mef-bridge # Integration bridge + 4D-Trichter
```

### 3. Basic Usage

```rust
use apollyon_mef_bridge::{
    State4D, Policy, Hyperbion,
    HDAGField, FunnelGraph, coupling_tick
};

fn main() {
    // Initialize components
    let policy = Policy::Explore.params();
    let hyperbion = Hyperbion::new();
    let mut hdag = HDAGField::new();
    let mut funnel = FunnelGraph::new();

    // Create input states
    let states = vec![
        State4D::new(1.0, 0.0, 0.0, 0.5),
        State4D::new(0.0, 1.0, 0.0, 0.5),
    ];

    // Execute coupling tick
    let result = coupling_tick(
        &states,
        0.0,           // time
        &policy,
        &hyperbion,
        &mut hdag,
        &mut funnel,
        true,          // generate proofs
    );

    println!("Next states: {:?}", result.states_4d_next);
    println!("Proof hash: {:?}", result.commit_hash);
}
```

## System Components

### 4D-Trichter

The core morphodynamic pattern compression system:

```rust
use apollyon_mef_bridge::trichter::{
    State4D, State5D, lift, proj_4d,
    Hyperbion, HDAGField, FunnelGraph,
    Policy, coupling_tick,
};

// Lift 4D to 5D
let s4d = State4D::new(1.0, 0.0, 0.0, 0.5);
let s5d = lift(&s4d, 0.0);  // omega = 0.0

// Project 5D back to 4D
let s4d_back = proj_4d(&s5d);
```

### Policies

Choose the right policy for your use case:

```rust
// Explore: High learning, discovery-focused
let explore = Policy::Explore.params();

// Exploit: Low decay, optimization-focused
let exploit = Policy::Exploit.params();

// Homeostasis: Adaptive, stability-focused
let homeostasis = Policy::Homeostasis {
    target_density: 0.5
}.params();
```

### Processing Pipeline

For multi-step evolution:

```rust
let mut states = vec![State4D::new(1.0, 0.0, 0.0, 0.5)];

for t in 0..100 {
    let result = coupling_tick(
        &states,
        t as f64,
        &policy,
        &hyperbion,
        &mut hdag,
        &mut funnel,
        false,  // skip proofs for performance
    );
    states = result.states_4d_next;
}

println!("Final node count: {}", funnel.node_count());
```

## Project Structure

```
dioniceOS/
├── apollyon_5d/           # 5D geometric-cognitive engine
├── infinity-ledger/       # Proof-carrying vector ledger
├── apollyon-mef-bridge/   # Integration + 4D-Trichter
├── overlay/               # Unified 5D Cube overlay
├── docs/                  # Documentation
│   ├── ARCHITECTURE.md    # System architecture
│   └── GETTING_STARTED.md # This file
├── README.md              # Project overview (English)
├── README_DE.md           # Project overview (German)
└── CHANGELOG.md           # Version history
```

## Documentation

- [README.md](../README.md) - Full project overview
- [ARCHITECTURE.md](./ARCHITECTURE.md) - Technical architecture
- [CHANGELOG.md](../CHANGELOG.md) - Release notes
- [4D_Trichter.pdf](../4D_Trichter.pdf) - Mathematical specification

## Troubleshooting

### Build Issues

```bash
# Clean and rebuild
cargo clean
cargo build --release --workspace
```

### Test Failures

```bash
# Run tests with output
cargo test --workspace -- --nocapture
```

### Version Check

```bash
rustc --version  # Should be 1.70+
cargo --version
```

## Next Steps

1. Read the [ARCHITECTURE.md](./ARCHITECTURE.md) for system understanding
2. Explore the [4D_Trichter.pdf](../4D_Trichter.pdf) specification
3. Check examples in `apollyon-mef-bridge/examples/`
4. Run benchmarks with `cargo bench`
