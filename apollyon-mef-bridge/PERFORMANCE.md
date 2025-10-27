# Performance Guide

This document provides guidance on optimizing and profiling the APOLLYON-MEF Bridge.

## Benchmarking

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench --package apollyon-mef-bridge

# Run specific benchmark
cargo bench --bench cognitive_engine_bench

# Save baseline
cargo bench --bench cognitive_engine_bench -- --save-baseline v1.0

# Compare against baseline
cargo bench --bench cognitive_engine_bench -- --baseline v1.0
```

### Benchmark Categories

1. **Single Processing**: Measures performance of processing individual inputs
   - Tests different integration times (0.1s to 2.0s)
   - Useful for understanding base performance

2. **Batch Processing**: Measures throughput of batch operations
   - Tests batch sizes from 1 to 20 inputs
   - Provides throughput metrics (items/second)

3. **Gate Configurations**: Compares performance across different gate configs
   - Default, strict, relaxed, and custom configurations
   - Helps identify configuration overhead

4. **Trajectory Sizes**: Performance vs trajectory length
   - Small (~10 states) to XLarge (~200 states)
   - Identifies scaling characteristics

## Hot Paths

### Identified Hot Paths

Based on the pipeline structure, these are the most performance-critical sections:

1. **5D Integration** (`integrate_5d`)
   - Heun's method integration loop
   - State validity checks
   - **Optimization**: Consider using SIMD for vector operations

2. **Spectral Analysis** (`analyze_spectrum`)
   - Trajectory traversal
   - Entropy computation
   - Centroid calculation
   - **Optimization**: Can be parallelized for large trajectories

3. **Route Selection** (`select_route_enhanced`)
   - SHA256 hashing
   - Topological mesh calculations
   - **Optimization**: Consider caching route calculations for identical states

4. **Proof-of-Resonance** (`compute_proof`)
   - Distance calculations
   - Field modulation sampling
   - **Optimization**: Pre-compute field modulation table for constant fields

### Optimization Strategies

#### 1. Memory Allocation

```rust
// Bad: Multiple allocations in loop
for state in trajectory {
    let mut temp = Vec::new();
    temp.push(state.x());
    // ... process
}

// Good: Pre-allocate outside loop
let mut temp = Vec::with_capacity(trajectory.len());
for state in trajectory {
    temp.clear();
    temp.push(state.x());
    // ... process
}
```

#### 2. Trajectory Processing

```rust
// Consider using rayon for parallel processing of large batches
use rayon::prelude::*;

let results: Vec<_> = inputs
    .par_iter()
    .map(|input| engine.process(input.clone()))
    .collect();
```

#### 3. Field Modulation Caching

For constant or slowly-varying resonance fields:

```rust
// Cache modulation values
let modulation_cache: Vec<Vec<f64>> = (0..5)
    .map(|i| {
        (0..5).map(|j| field.modulation(t, i, j)).collect()
    })
    .collect();
```

## Memory Profiling

### Using `cargo-flamegraph`

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --bench cognitive_engine_bench

# Open flamegraph.svg in browser
```

### Using `heaptrack`

```bash
# Run with heaptrack (Linux only)
heaptrack cargo bench --bench cognitive_engine_bench

# Analyze results
heaptrack_gui heaptrack.cognitive_engine_bench.*
```

### Using `valgrind` (Linux)

```bash
# Memory leak detection
valgrind --leak-check=full cargo test

# Cachegrind profiling
valgrind --tool=cachegrind cargo bench --bench cognitive_engine_bench
```

### Memory Usage Characteristics

#### Trajectory Storage

- Each `State5D`: 40 bytes (5 × f64)
- Typical trajectory (100 states): ~4 KB
- Large trajectory (1000 states): ~40 KB

#### Batch Processing

- Memory scales linearly with batch size
- Each `CognitiveOutput`: ~4-40 KB (depending on trajectory length)
- Batch of 100 outputs with 100-state trajectories: ~400 KB - 4 MB

#### Recommendations

1. **Stream Processing**: For very large batches, process in chunks
   ```rust
   for chunk in inputs.chunks(100) {
       let results = engine.process_batch(chunk.to_vec());
       // Store results before processing next chunk
   }
   ```

2. **Trajectory Pruning**: If full trajectory not needed, consider sampling
   ```rust
   // Keep every Nth state instead of all states
   let sampled_trajectory: Vec<_> = trajectory
       .iter()
       .step_by(10)
       .cloned()
       .collect();
   ```

3. **Use References**: Avoid unnecessary clones
   ```rust
   // Bad
   fn process_output(output: CognitiveOutput) { }

   // Good
   fn process_output(output: &CognitiveOutput) { }
   ```

## Async Performance

### Parallelism Tuning

```rust
// Tune parallelism based on your workload
let parallelism = num_cpus::get(); // Default
let parallelism = 4; // Fixed number
let parallelism = num_cpus::get() * 2; // Oversubscribe for I/O-heavy tasks

let result = async_engine
    .process_batch_parallel(inputs, Some(parallelism))
    .await;
```

### Tokio Runtime Configuration

```rust
use tokio::runtime::Builder;

let runtime = Builder::new_multi_thread()
    .worker_threads(4)
    .thread_name("apollyon-mef-worker")
    .thread_stack_size(3 * 1024 * 1024)
    .build()
    .unwrap();

runtime.block_on(async {
    // Your async code
});
```

## Storage Performance

### Ledger Storage

- **Write Performance**: ~10-100 writes/second (disk-bound)
- **Optimization**: Use batch writes when possible
- **Consideration**: SSD vs HDD makes significant difference

### Memory Storage

- **Write Performance**: ~100,000+ writes/second (RAM-bound)
- **Use Case**: Testing, caching, temporary results
- **Limitation**: No persistence, limited by available RAM

## Performance Targets

### Single Input Processing

- Small trajectory (10 states): < 1 ms
- Medium trajectory (100 states): < 5 ms
- Large trajectory (1000 states): < 50 ms

### Batch Processing

- Throughput: > 100 inputs/second (medium trajectories)
- Latency: < 10 ms average per input
- Success Rate: > 99%

### Memory Usage

- Baseline (empty engine): < 1 MB
- Per processing: < 100 KB overhead
- Batch of 100: < 10 MB total

## Profiling Checklist

- [ ] Run benchmarks and establish baseline
- [ ] Profile CPU usage with flamegraph
- [ ] Profile memory allocation with heaptrack
- [ ] Identify hot paths (>10% CPU time)
- [ ] Optimize hot paths
- [ ] Re-run benchmarks and compare
- [ ] Document optimization results

## Advanced Techniques

### SIMD Acceleration

For 5D vector operations, consider using SIMD:

```rust
use std::simd::f64x4;

// Example: Vectorized distance calculation
fn simd_distance(a: &State5D, b: &State5D) -> f64 {
    // Requires nightly Rust with portable_simd feature
    // Implementation left as exercise
}
```

### GPU Acceleration

For very large batches or complex fields:

```rust
// Consider using wgpu or CUDA for GPU acceleration
// Particularly useful for:
// - Large batch processing (>1000 inputs)
// - Complex resonance field calculations
// - Spectral analysis on very long trajectories
```

### Compile-Time Optimization

```toml
# Cargo.toml
[profile.release]
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization, slower compile
opt-level = 3           # Maximum optimization
strip = true            # Strip symbols for smaller binary

[profile.bench]
inherits = "release"
debug = true            # Keep debug info for profiling
```

## Monitoring in Production

### Metrics to Track

1. **Processing Latency**
   - p50, p95, p99 latencies
   - Track by trajectory size

2. **Throughput**
   - Inputs processed per second
   - Success/failure rate

3. **Resource Usage**
   - CPU utilization
   - Memory usage
   - Storage I/O

4. **Gate Decisions**
   - FIRE vs HOLD ratio
   - May indicate tuning needs

### Example Metrics Collection

```rust
use std::time::Instant;

let start = Instant::now();
let result = engine.process(input);
let duration = start.elapsed();

// Log metrics
tracing::info!(
    "Processing completed in {:?}, decision: {:?}",
    duration,
    result.as_ref().map(|r| r.gate_decision)
);
```

## Troubleshooting Performance Issues

### Slow Processing

1. Check trajectory length (`t_final / dt`)
2. Profile to identify bottleneck
3. Consider using smaller integration steps
4. Use batch processing for multiple inputs

### High Memory Usage

1. Check trajectory sizes in outputs
2. Consider pruning/sampling trajectories
3. Process in smaller batches
4. Use streaming for large datasets

### Low Batch Throughput

1. Increase parallelism for async batches
2. Profile for contention (locks, etc.)
3. Consider using rayon for CPU-bound work
4. Check I/O if using ledger storage

## Further Reading

- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Criterion.rs Guide](https://bheisler.github.io/criterion.rs/book/)
- [Tokio Performance Tuning](https://tokio.rs/tokio/topics/performance)
- [SIMD in Rust](https://rust-lang.github.io/packed_simd/perf-guide/)
