# APOLLYON-MEF Bridge Tutorials

Complete tutorials for getting started and advanced usage of the APOLLYON-MEF Bridge.

## Table of Contents

1. [Getting Started](#tutorial-1-getting-started)
2. [Understanding the Pipeline](#tutorial-2-understanding-the-pipeline)
3. [Custom Gate Configurations](#tutorial-3-custom-gate-configurations)
4. [Custom Resonance Fields](#tutorial-4-custom-resonance-fields)
5. [Batch Processing](#tutorial-5-batch-processing)
6. [Async Processing](#tutorial-6-async-processing)
7. [Storage Integration](#tutorial-7-storage-integration)
8. [Performance Optimization](#tutorial-8-performance-optimization)
9. [Error Handling](#tutorial-9-error-handling)
10. [Production Deployment](#tutorial-10-production-deployment)

---

## Tutorial 1: Getting Started

### Prerequisites

```toml
# Add to your Cargo.toml
[dependencies]
apollyon-mef-bridge = { path = "../apollyon-mef-bridge" }
core_5d = { path = "../apollyon_5d/core" }
tokio = { version = "1", features = ["full"] }
```

### Your First Processing

```rust
use apollyon_mef_bridge::unified::{CognitiveInput, UnifiedCognitiveEngine};
use core_5d::{State5D, SystemParameters};

fn main() {
    // 1. Create the engine
    let mut engine = UnifiedCognitiveEngine::new();

    // 2. Prepare input
    let input = CognitiveInput {
        initial_state: State5D::new(1.0, 0.5, 0.3, 0.2, 0.1),
        parameters: SystemParameters::default(),
        t_final: 1.0,
        tic_id: "TIC-001".to_string(),
        seed: "my_seed".to_string(),
        seed_path: "MEF/domain/stage/0001".to_string(),
    };

    // 3. Process
    match engine.process(input) {
        Ok(output) => {
            println!("Success!");
            println!("Trajectory: {} states", output.trajectory.len());
            println!("Gate: {:?}", output.gate_decision);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Understanding the Input

- **initial_state**: Starting point in 5D space (x, y, z, ψ, ω)
- **parameters**: System dynamics parameters (κ, γ, Φ₀, ε)
- **t_final**: Integration time (longer = more states)
- **tic_id**: Unique identifier for this computation
- **seed**: Routing seed for deterministic path selection
- **seed_path**: HD-style derivation path

### Understanding the Output

```rust
let output: CognitiveOutput = engine.process(input)?;

// Trajectory: The path through 5D space
println!("States: {}", output.trajectory.len());
for (i, state) in output.trajectory.iter().take(3).enumerate() {
    println!("  State {}: ({:.3}, {:.3}, {:.3}, {:.3}, {:.3})",
             i, state.x(), state.y(), state.z(), state.psi(), state.omega());
}

// Spectral signature
println!("Spectral: ψ={:.3}, ρ={:.3}, ω={:.3}",
         output.spectral_signature.psi,
         output.spectral_signature.rho,
         output.spectral_signature.omega);

// Route selection
println!("Route: {}", output.route.route_id);

// Proof of resonance
println!("PoR: δPI={:.6}, Φ={:.3}, δV={:.6}",
         output.proof.delta_pi,
         output.proof.phi,
         output.proof.delta_v);

// Gate decision
println!("Gate: {:?}", output.gate_decision);

// Knowledge object (if created)
if let Some(knowledge) = &output.knowledge {
    println!("MEF ID: {}", knowledge.mef_id);
}
```

---

## Tutorial 2: Understanding the Pipeline

### The 8 Pipeline Phases

```rust
use apollyon_mef_bridge::unified::UnifiedCognitiveEngine;

fn explore_pipeline() {
    let mut engine = UnifiedCognitiveEngine::new();
    let output = engine.process(input)?;

    println!("=== Pipeline Phases ===\n");

    // Phase 1: APOLLYON-5D Integration
    println!("1. APOLLYON Integration");
    println!("   Generated {} states", output.trajectory.len());

    // Phase 2: Spectral Analysis
    println!("2. Spectral Analysis");
    println!("   Signature: {:?}", output.spectral_signature);

    // Phase 3: State Conversion
    println!("3. State Conversion");
    println!("   5D → Spiral coordinates");

    // Phase 4: Route Selection
    println!("4. Route Selection");
    println!("   Selected route: {}", output.route.route_id);
    println!("   Mesh score: {:.3}", output.route.mesh_score);

    // Phase 5: Knowledge Derivation
    println!("5. Knowledge Derivation");
    if let Some(k) = &output.knowledge {
        println!("   MEF ID: {}", k.mef_id);
        println!("   TIC: {}, Route: {}", k.tic_id, k.route_id);
    }

    // Phase 6: Proof-of-Resonance
    println!("6. Proof-of-Resonance");
    println!("   δPI (path invariance): {:.6}", output.proof.delta_pi);
    println!("   Φ (alignment): {:.3}", output.proof.phi);
    println!("   δV (Lyapunov delta): {:.6}", output.proof.delta_v);
    println!("   Valid: {}", output.proof.por_valid);

    // Phase 7: Gate Evaluation
    println!("7. Gate Evaluation");
    println!("   Decision: {:?}", output.gate_decision);
    println!("   Logic: (PoR valid) ∧ (δPI ≤ ε) ∧ (Φ ≥ φ) ∧ (δV < 0)");

    // Phase 8: Storage (conditional)
    println!("8. Storage");
    match output.gate_decision {
        GateDecision::FIRE => println!("   → Store to ledger"),
        GateDecision::HOLD => println!("   → Skip storage"),
    }
}
```

### Pipeline Visualization

```
Input → Integration → Spectral → Conversion → Routing
    → Knowledge → PoR → Gate → Storage → Output

Time: ~1-10ms per input (depends on trajectory length)
```

---

## Tutorial 3: Custom Gate Configurations

### Default vs Custom Configurations

```rust
use apollyon_mef_bridge::unified::{GateConfig, UnifiedCognitiveEngine};

fn compare_configurations() {
    // Default configuration
    let default_config = GateConfig::default();
    println!("Default: ε={}, φ={}, resonance={}",
             default_config.epsilon,
             default_config.phi_threshold,
             default_config.resonance_strength);

    // Strict configuration (harder to FIRE)
    let strict_config = GateConfig::strict();
    let strict_engine = UnifiedCognitiveEngine::new_with_config(strict_config);

    // Relaxed configuration (easier to FIRE)
    let relaxed_config = GateConfig::relaxed();
    let relaxed_engine = UnifiedCognitiveEngine::new_with_config(relaxed_config);

    // Custom configuration
    let custom_config = GateConfig::new(
        0.15,  // epsilon: path invariance threshold
        0.6,   // phi_threshold: alignment threshold
        0.85   // resonance_strength: field strength
    );
    let custom_engine = UnifiedCognitiveEngine::new_with_config(custom_config);
}
```

### Understanding Gate Parameters

#### Epsilon (ε) - Path Invariance Threshold

Controls how much the state can move between steps:

- **Lower** (0.05): Strict, requires very stable transitions
- **Default** (0.1): Balanced
- **Higher** (0.2): Relaxed, allows more movement

```rust
// Strict: Only very stable trajectories FIRE
GateConfig::new(0.05, 0.5, 0.8)

// Relaxed: More trajectories can FIRE
GateConfig::new(0.2, 0.5, 0.8)
```

#### Phi Threshold (φ) - Alignment Threshold

Controls resonance alignment requirement:

- **Lower** (0.3): Easier to achieve alignment
- **Default** (0.5): Balanced
- **Higher** (0.7): Requires strong alignment

```rust
// Easy alignment
GateConfig::new(0.1, 0.3, 0.8)

// Strict alignment
GateConfig::new(0.1, 0.7, 0.8)
```

#### Resonance Strength

Controls the resonance field modulation:

- **Lower** (0.6): Weaker resonance effects
- **Default** (0.8): Balanced
- **Higher** (0.9): Stronger resonance effects

### Dynamic Configuration

```rust
// Start with default
let mut engine = UnifiedCognitiveEngine::new();

// Process some inputs
let result1 = engine.process(input1)?;

// Adjust configuration based on results
if result1.gate_decision == GateDecision::HOLD {
    // Try with relaxed config
    let relaxed = GateConfig::relaxed();
    engine.set_gate_config(relaxed);
}

let result2 = engine.process(input2)?;
```

---

## Tutorial 4: Custom Resonance Fields

### Built-in Fields

```rust
use bridge::{ConstantResonanceField, OscillatoryResonanceField};

// Constant field (default)
let constant = Box::new(ConstantResonanceField::new(0.8));
let engine1 = UnifiedCognitiveEngine::new_with_field(constant);

// Oscillatory field (time-varying)
let oscillatory = Box::new(OscillatoryResonanceField::new(
    0.2,  // amplitude
    1.0,  // frequency
    0.0   // phase
));
let engine2 = UnifiedCognitiveEngine::new_with_field(oscillatory);
```

### Creating Custom Fields

```rust
use bridge::ResonanceField;

/// Exponential decay field
struct DecayField {
    initial: f64,
    rate: f64,
}

impl ResonanceField for DecayField {
    fn modulation(&self, t: f64, _i: usize, _j: usize) -> f64 {
        self.initial * (-self.rate * t).exp()
    }
}

// Use it
let field = Box::new(DecayField { initial: 1.0, rate: 0.5 });
let engine = UnifiedCognitiveEngine::new_with_field(field);
```

### Node-Specific Fields

```rust
/// Different coupling for different node pairs
struct CustomCouplingField {
    strengths: [[f64; 5]; 5],
}

impl ResonanceField for CustomCouplingField {
    fn modulation(&self, _t: f64, i: usize, j: usize) -> f64 {
        if i < 5 && j < 5 {
            self.strengths[i][j]
        } else {
            1.0
        }
    }
}
```

### Adaptive Fields

```rust
/// Field that adapts based on state
struct AdaptiveField {
    base: f64,
    sensitivity: f64,
}

impl ResonanceField for AdaptiveField {
    fn modulation(&self, t: f64, i: usize, j: usize) -> f64 {
        // Modulate based on time and nodes
        let factor = 1.0 + self.sensitivity * (t * (i + j) as f64).sin();
        self.base * factor
    }

    fn reset(&mut self) {
        // Reset any internal state if needed
    }
}
```

---

## Tutorial 5: Batch Processing

### Sequential Batch

```rust
use apollyon_mef_bridge::unified::{UnifiedCognitiveEngine, CognitiveInput};

fn process_batch() {
    let mut engine = UnifiedCognitiveEngine::new();

    // Create multiple inputs
    let inputs: Vec<CognitiveInput> = (1..=10)
        .map(|i| create_input(i))
        .collect();

    // Process batch
    let result = engine.process_batch(inputs);

    println!("Batch Results:");
    println!("  Total: {}", result.total_count());
    println!("  Success: {}", result.success_count());
    println!("  Failed: {}", result.failure_count());
    println!("  Success rate: {:.1}%", result.success_rate());
    println!("  Total time: {:.3}s", result.total_time);
    println!("  Avg time: {:.3}s", result.avg_time);

    // Access successful outputs
    for output in &result.successes {
        println!("  ✓ {}: {:?}",
                 output.knowledge.as_ref().unwrap().tic_id,
                 output.gate_decision);
    }

    // Handle failures
    for (index, error) in &result.failures {
        println!("  ✗ Input {}: {}", index, error);
    }
}
```

### Streaming Batch

For very large datasets, process in chunks:

```rust
fn process_large_dataset(inputs: Vec<CognitiveInput>) {
    let mut engine = UnifiedCognitiveEngine::new();
    const CHUNK_SIZE: usize = 100;

    for (i, chunk) in inputs.chunks(CHUNK_SIZE).enumerate() {
        println!("Processing chunk {}...", i + 1);

        let result = engine.process_batch(chunk.to_vec());

        println!("  Success: {}/{}",
                 result.success_count(),
                 result.total_count());

        // Store or process results before next chunk
        handle_results(&result);
    }
}
```

---

## Tutorial 6: Async Processing

### Basic Async

```rust
use apollyon_mef_bridge::unified::AsyncUnifiedCognitiveEngine;

#[tokio::main]
async fn main() {
    let mut engine = AsyncUnifiedCognitiveEngine::new();

    let input = create_input(1);

    // Async processing
    match engine.process_async(input).await {
        Ok(output) => println!("Success!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Parallel Batch

```rust
#[tokio::main]
async fn main() {
    let engine = AsyncUnifiedCognitiveEngine::new();

    let inputs: Vec<_> = (1..=100).map(create_input).collect();

    // Process in parallel with 8 workers
    let result = engine.process_batch_parallel(inputs, Some(8)).await;

    println!("Processed {} inputs in {:.2}s",
             result.total_count(),
             result.total_time);
    println!("Throughput: {:.1} items/sec",
             result.total_count() as f64 / result.total_time);
}
```

### Tuning Parallelism

```rust
// Auto: Use number of CPU cores
let result1 = engine.process_batch_parallel(inputs.clone(), None).await;

// Fixed: Use specific number
let result2 = engine.process_batch_parallel(inputs.clone(), Some(4)).await;

// Oversubscribe: More tasks than cores (for I/O-bound work)
let cores = num_cpus::get();
let result3 = engine.process_batch_parallel(inputs, Some(cores * 2)).await;
```

### Concurrent Processing

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    // Spawn multiple async tasks
    for i in 1..=10 {
        let handle = task::spawn(async move {
            let mut engine = AsyncUnifiedCognitiveEngine::new();
            let input = create_input(i);
            engine.process_async(input).await
        });
        handles.push(handle);
    }

    // Wait for all to complete
    for handle in handles {
        match handle.await {
            Ok(Ok(output)) => println!("✓"),
            Ok(Err(e)) => eprintln!("Processing error: {}", e),
            Err(e) => eprintln!("Task error: {}", e),
        }
    }
}
```

---

## Tutorial 7: Storage Integration

### Memory Storage (Development)

```rust
use apollyon_mef_bridge::storage::{MemoryStorage, StorageBackend};

#[tokio::main]
async fn main() {
    let mut storage = MemoryStorage::new();

    // Process and store
    let output = engine.process(input)?;
    let id = storage.store(&output).await?;

    println!("Stored with ID: {}", id);

    // Retrieve
    let retrieved = storage.retrieve(&id).await?;

    // Clear when done
    storage.clear();
}
```

### Ledger Storage (Production)

```rust
use apollyon_mef_bridge::storage::LedgerStorage;

#[tokio::main]
async fn main() {
    let mut storage = LedgerStorage::new("./ledger")?;

    // Only FIRE decisions are stored
    let output = engine.process(input)?;

    if matches!(output.gate_decision, GateDecision::FIRE) {
        let id = storage.store(&output).await?;
        println!("Stored to ledger: {}", id);
    }
}
```

### Storage with Batch Processing

```rust
#[tokio::main]
async fn main() {
    let mut engine = UnifiedCognitiveEngine::new();
    let mut storage = LedgerStorage::new("./ledger")?;

    let batch_result = engine.process_batch(inputs);

    // Store all FIRE decisions
    for output in &batch_result.successes {
        if matches!(output.gate_decision, GateDecision::FIRE) {
            storage.store(output).await?;
        }
    }

    // Check statistics
    let stats = storage.stats().await?;
    println!("Stored {} items", stats.total_items);
}
```

### Health Checks

```rust
async fn monitor_storage(storage: &impl StorageBackend) {
    match storage.health_check().await {
        Ok(true) => println!("Storage healthy"),
        Ok(false) => eprintln!("Storage unhealthy!"),
        Err(e) => eprintln!("Health check failed: {}", e),
    }

    if let Ok(stats) = storage.stats().await {
        println!("Storage stats:");
        println!("  Items: {}", stats.total_items);
        println!("  Size: {} MB", stats.total_size_bytes / 1_000_000);
        println!("  Success rate: {:.1}%",
                 100.0 * stats.successful_writes as f64 /
                 (stats.successful_writes + stats.failed_writes) as f64);
    }
}
```

---

## Tutorial 8: Performance Optimization

### Benchmarking

```bash
# Run benchmarks
cargo bench --bench cognitive_engine_bench

# Compare configurations
cargo bench -- --baseline default
cargo bench -- --baseline optimized
```

### Memory Optimization

```rust
// Bad: Creates many allocations
for i in 0..1000 {
    let input = create_input(i);
    let output = engine.process(input)?;
    // output dropped here
}

// Good: Reuse allocations where possible
let mut engine = UnifiedCognitiveEngine::new();
for chunk in inputs.chunks(100) {
    let batch_result = engine.process_batch(chunk.to_vec());
    handle_and_clear_results(batch_result);
}
```

### Trajectory Sampling

```rust
// If full trajectory not needed, sample it
fn sample_trajectory(trajectory: &[State5D], step: usize) -> Vec<State5D> {
    trajectory.iter()
        .step_by(step)
        .cloned()
        .collect()
}

// Use every 10th state instead of all
let sampled = sample_trajectory(&output.trajectory, 10);
```

### Parallel Processing

```rust
use rayon::prelude::*;

// Process multiple independent inputs in parallel
let results: Vec<_> = inputs
    .par_iter()
    .map(|input| {
        let mut engine = UnifiedCognitiveEngine::new();
        engine.process(input.clone())
    })
    .collect();
```

---

## Tutorial 9: Error Handling

### Error Types

```rust
use apollyon_mef_bridge::unified::cognitive_engine::CognitiveError;

match engine.process(input) {
    Ok(output) => handle_success(output),

    Err(CognitiveError::IntegrationError(msg)) => {
        eprintln!("Integration failed: {}", msg);
        // Try with different parameters
    }

    Err(CognitiveError::InvalidState(msg)) => {
        eprintln!("Invalid state: {}", msg);
        // Check initial state validity
    }

    Err(CognitiveError::SpectralAnalysisError(msg)) => {
        eprintln!("Spectral analysis failed: {}", msg);
        // Try with longer trajectory
    }

    Err(CognitiveError::RouteSelectionError(msg)) => {
        eprintln!("Route selection failed: {}", msg);
        // Try with different seed
    }

    Err(CognitiveError::EmptyTrajectory) => {
        eprintln!("No trajectory generated");
        // Increase t_final
    }
}
```

### Retry Logic

```rust
fn process_with_retry(
    engine: &mut UnifiedCognitiveEngine,
    input: CognitiveInput,
    max_retries: usize,
) -> Result<CognitiveOutput, CognitiveError> {
    let mut attempts = 0;

    loop {
        match engine.process(input.clone()) {
            Ok(output) => return Ok(output),
            Err(e) => {
                attempts += 1;
                if attempts >= max_retries {
                    return Err(e);
                }
                eprintln!("Attempt {} failed, retrying...", attempts);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
```

### Graceful Degradation

```rust
fn process_with_fallback(
    input: CognitiveInput
) -> Result<CognitiveOutput, CognitiveError> {
    // Try with default config
    let mut engine = UnifiedCognitiveEngine::new();

    match engine.process(input.clone()) {
        Ok(output) => Ok(output),
        Err(_) => {
            // Fallback to relaxed config
            let relaxed = GateConfig::relaxed();
            engine.set_gate_config(relaxed);
            engine.process(input)
        }
    }
}
```

---

## Tutorial 10: Production Deployment

### Production Checklist

```rust
use apollyon_mef_bridge::{
    unified::{UnifiedCognitiveEngine, GateConfig},
    storage::LedgerStorage,
};
use tracing::{info, error};

struct ProductionEngine {
    engine: UnifiedCognitiveEngine,
    storage: LedgerStorage,
}

impl ProductionEngine {
    fn new(ledger_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 1. Configure for production
        let config = GateConfig::default(); // or load from config file

        // 2. Create engine
        let engine = UnifiedCognitiveEngine::new_with_config(config);

        // 3. Setup storage
        let storage = LedgerStorage::new(ledger_path)?;

        Ok(Self { engine, storage })
    }

    async fn process_and_store(
        &mut self,
        input: CognitiveInput,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Log input
        info!("Processing TIC: {}", input.tic_id);

        // Process
        let output = self.engine.process(input)?;

        // Log result
        info!("Decision: {:?}", output.gate_decision);

        // Store if FIRE
        let result = if matches!(output.gate_decision, GateDecision::FIRE) {
            let id = self.storage.store(&output).await?;
            info!("Stored to ledger: {}", id);
            id
        } else {
            info!("HOLD decision, not stored");
            output.knowledge.as_ref().unwrap().mef_id.clone()
        };

        Ok(result)
    }
}
```

### Monitoring

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

struct Metrics {
    total_processed: AtomicU64,
    total_fire: AtomicU64,
    total_hold: AtomicU64,
    total_errors: AtomicU64,
}

impl Metrics {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            total_processed: AtomicU64::new(0),
            total_fire: AtomicU64::new(0),
            total_hold: AtomicU64::new(0),
            total_errors: AtomicU64::new(0),
        })
    }

    fn record_success(&self, decision: GateDecision) {
        self.total_processed.fetch_add(1, Ordering::Relaxed);
        match decision {
            GateDecision::FIRE => {
                self.total_fire.fetch_add(1, Ordering::Relaxed);
            }
            GateDecision::HOLD => {
                self.total_hold.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    fn record_error(&self) {
        self.total_errors.fetch_add(1, Ordering::Relaxed);
    }

    fn report(&self) {
        let total = self.total_processed.load(Ordering::Relaxed);
        let fire = self.total_fire.load(Ordering::Relaxed);
        let hold = self.total_hold.load(Ordering::Relaxed);
        let errors = self.total_errors.load(Ordering::Relaxed);

        info!("Metrics:");
        info!("  Total processed: {}", total);
        info!("  FIRE: {} ({:.1}%)", fire, 100.0 * fire as f64 / total as f64);
        info!("  HOLD: {} ({:.1}%)", hold, 100.0 * hold as f64 / total as f64);
        info!("  Errors: {}", errors);
    }
}
```

### Configuration Management

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    gate: GateSettings,
    storage: StorageSettings,
    performance: PerformanceSettings,
}

#[derive(Serialize, Deserialize)]
struct GateSettings {
    epsilon: f64,
    phi_threshold: f64,
    resonance_strength: f64,
}

#[derive(Serialize, Deserialize)]
struct StorageSettings {
    ledger_path: String,
    backup_enabled: bool,
}

#[derive(Serialize, Deserialize)]
struct PerformanceSettings {
    batch_size: usize,
    parallelism: usize,
}

fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

### Logging

```rust
use tracing_subscriber;

fn setup_logging() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
}

fn main() {
    setup_logging();

    // Rest of your code
}
```

---

## Next Steps

1. **Experiment**: Try the examples and modify them
2. **Profile**: Run benchmarks on your workload
3. **Customize**: Create custom fields and configurations
4. **Deploy**: Follow the production checklist
5. **Monitor**: Track metrics and performance

## Further Resources

- [Architecture Documentation](./ARCHITECTURE.md)
- [Performance Guide](./PERFORMANCE.md)
- [API Documentation](https://docs.rs/apollyon-mef-bridge)
- [Examples](./examples/)

## Getting Help

- Check existing examples first
- Review the architecture documentation
- Run the tests to understand behavior
- Profile performance issues before optimizing

Happy coding! 🚀
