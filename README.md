# Mirror256 Hash Function Rust Implementation

This is a Rust implementation of the experimental Mirror256 hash function designed for optical/quantum computers. The algorithm is based on Toffoli and Fredkin gates organized in a zigzag pattern.

## Features

- Pure Rust implementation of the Mirror256 hash algorithm
- 256-bit hash output
- Uses Toffoli and Fredkin gates in 128 layers with 2 sublayers each
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

## Hashing Algorithm
The Mirror-Hash algorithm has the following characteristics:

- Standard 256-bit input
- 128 layers of gates
- Each layer has 2 sublayers of Toffoli or Fredkin gates in zig-zag fashion
- The symmetry (mirrored or not) and type of gate (Toffoli or Fredkin) is determined by the previous block (called layer encoding here) of the hash
- XOR operation with the current layer encoding to avoid 0-to-0 hashes

Here's a ASCI diagram in Markdown for the gate grid specification:

| Gate type | Symbol | Encoding |
|-----------|--------|----------|
| Toffoli   |   #    |    00    |
| Mirrored Toffoli   |   #̅   |    01    |
| Fredkin   |   @    |    10    |
| Mirrored Fredkin   |   @̅   |    11    |

| Layer | Column 1 | Column 2 | Column 3 | Column 4 | Column 5 | Column 6 | Column 7 | Column 8 |
|-------|----------|----------|----------|----------|----------|----------|----------|----------|
|   1   |   ###    |   @@@    |   ###    |   @@@    |   ###    |   @@@    |   ###    |   @@@    |
|   2   |   ###    |          |   ###    |          |   ###    |          |   ###    |          |
|   3   |   ###    |   @@@    |   ###    |   @@@    |   ###    |   @@@    |   ###    |   @@@    |
|   4   |          |   @@@    |          |   @@@    |          |   @@@    |          |   @@@    |
|   5   |   ###    |   @@@    |   ###    |   @@@    |   ###    |   @@@    |   ###    |   @@@    |
|   6   |   ###    |          |   ###    |          |   ###    |          |   ###    |          |
|   7   |   ###    |   @@@    |   ###    |   @@@    |   ###    |   @@@    |   ###    |   @@@    |
|   8   |          |   @@@    |          |   @@@    |          |   @@@    |          |   @@@    |
|  ...  |   ...    |    ...   |    ...   |    ...   |    ...   |    ...   |    ...   |    ...   |
|  128  |   ###    |   @@@    |   ###    |   @@@    |   ###    |   @@@    |   ###    |   @@@    |

## References

Original Python implementation: [mirror-hash](https://github.com/jio-gl/python-mirror-hash)

## License

Apache License 