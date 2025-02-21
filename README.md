# Shamir Secret Sharing Implementation

A Rust implementation of Shamir's Secret Sharing scheme, which allows splitting a secret into multiple shares and reconstructing it only when a minimum threshold of shares is present.

## Description

This project demonstrates the use of Shamir's Secret Sharing algorithm to:

- Generate a random 16-byte secret
- Split it into multiple shares
- Reconstruct the original secret from a threshold number of shares

The implementation uses the `sharks` crate for the core secret sharing functionality and `rand` for generating random secrets.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository

2. Build the project

```bash
cargo build
```

### Usage

Run the program using:

```bash
cargo run
```

The program will:

1. Generate a random 16-byte secret
2. Split it into 10 shares
3. Reconstruct the secret using these shares
4. Verify the reconstruction matches the original

## Dependencies

- `sharks` (v0.5.0) - Implementation of Shamir's Secret Sharing
- `rand` (v0.9.0) - Random number generation

## License

This project is licensed under the MIT License - see the LICENSE file for details

## Acknowledgments

- [sharks](https://crates.io/crates/sharks) crate for providing the secret sharing implementation
