# AES GCM Proxy Re-encryption

This Rust library implements proxy re-encryption for symmetric ciphers, focusing on AES in Galois/Counter Mode (GCM). The implementation is based on the principles outlined in the document "Proxy Re-encryption for Symmetric Ciphers" by Debrup Chatterjee.

## Features

- **GCTR (Galois/Counter Mode)**: Implements counter mode operation for encryption without producing an authentication tag.
- **GHASH**: Computes the authentication tag using Galois field arithmetic.
- **GCM-AE**: Provides authenticated encryption in GCM, producing ciphertext and an authentication tag.

## Components

### GCTR
The GCTR operation is defined as follows:
For each plaintext block \(P_i\), the corresponding ciphertext block \(C_i\) is computed as \(P_i \oplus E(K, Y_i)\), where \(E(K, X)\) denotes the encryption of block \(X\) under key \(K\), and \(Y_i\) is the incremented counter value.

### GHASH
GHASH is used to compute the authentication tag, which ensures the integrity and authenticity of the ciphertext and any additional authenticated data (AAD).

### GCM-AE
The GCM authenticated encryption function combines GCTR and GHASH to provide secure encryption and authentication.

## Proxy Re-Encryption Method
The library includes functionality to re-encrypt ciphertext from one key to another. This involves computing a new ciphertext \(C^B\) and tag \(T^B\) for a different key \(K_B\) while ensuring the integrity of the original plaintext.

## Usage

This library, uses the following dependencies in Cargo.toml file:

```toml
[dependencies]
aes-gcm = "0.10.3"
rand = "0.8.5"
aead = "0.5"
generic-array = "0.14.4"
universal-hash = "0.5.1"
hex = "0.4.3"
aes = "0.8"
cipher = "0.4"
ctr = "0.9"
ghash = "0.5.0"
hex-literal = "0.4.1"
subtle = { version = "2", default-features = false }
zeroize = { version = "1", optional = true, default-features = false }
```

## Testing
Run the unit tests with the following command:

```bash
cargo test -- --nocapture
```