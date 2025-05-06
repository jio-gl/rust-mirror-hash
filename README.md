# Mirror256 Hash Function Rust Implementation

This is a Rust implementation of the experimental Mirror256 hash function designed for optical/quantum computers. The algorithm is based on Toffoli and Fredkin gates organized in a zigzag pattern.

## Features

- Pure Rust implementation of the Mirror256 hash algorithm
- 256-bit hash output
- Uses Toffoli and Fredkin gates in 64 layers with 2 sublayers each
- Provably reversible (bijective) hash function
- Optional standard state initialization using cubic roots of primes

## Usage

```rust
use mirror_hash::Mirror256;

// Create a new hasher
let mut hasher = Mirror256::new(None, None, None, true);

// Update with data
hasher.update("This is a test message");

// Get the digest
let digest = hasher.hexdigest();
println!("Hash: {}", digest);
```

## Benchmarks

Performance measurements on a MacBook Air M2:

| Operation               | Time        |
|-------------------------|-------------|
| Hash empty string       | 37.84 µs    |
| Hash short string       | 602.86 µs   |
| Hash medium string      | 1.75 ms     |
| Hash long string (1KB)  | 18.20 ms    |
| Multiple updates        | 1.64 ms     |

Throughput: ~411 hashes per second for random 32-byte inputs.

## Background

The Mirror256 hash function is an experimental hash algorithm designed for optical/quantum computers. It processes data through multiple layers of quantum-inspired gates (Toffoli and Fredkin) arranged in a zigzag pattern. Each gate's type and symmetry is determined by the state of previous hash operations.

## References

Original Python implementation: [mirror-hash](https://github.com/jio-gl/mirror-hash)

## License

Apache License 