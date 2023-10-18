# `noir_rs`: A Pure Rust Implementation of the Noir zkSNARK Proving Scheme

Welcome to `noir_rs`, a Rust-centric package designed for users seeking a straightforward, lightweight interface to generate and verify zkSNARK proofs without any WebAssembly (WASM) dependency.

## Key Highlights:
- **Pure Rust**: Capitalize on the safety and concurrency features Rust offers with this entire implementation.
- **Barretenberg Backend**: Leverages an up-to-date Barretenberg backend optimized for Rust bindings.
- **Native FFI Bindings**: Provides foreign function interface (FFI) bindings to the compiled library for a seamless interaction with the Barretenberg backend.
- **Proof Generation & Verification**: Enables zkSNARK proof generation and verification using the Barretenberg backend.
- **ACVM Integration**: Incorporates the ACVM (Arbitrary Computation Virtual Machine) package, allowing execution of circuits and solving for witnesses.

## Motivation:

zkSNARKs are rapidly gaining traction in the tech world, opening doors to various applications. However, the existing tools and platforms often come with complexities and compatibility challenges, especially on mobile platforms. The key motivations behind `noir_rs` include:

- **Rust-centric Design**: With Rust's performance, safety, and concurrency features, it provides a robust foundation for a zkSNARK platform.
- **Mobile Compatibility**: By avoiding WASM and creating a Rust-native solution, `noir_rs` is optimized for mobile devices.
- **Ease of Use**: The primary goal is to give developers a straightforward and lightweight toolset for generating and verifying zkSNARK proofs without getting bogged down by intricate configurations or dependencies.

## How it Works:

1. **Barretenberg Backend**: `noir_rs` employs a Barretenberg backend adjusted for Rust bindings, which plays a pivotal role in zkSNARK proof generation, verification, and also serves as the ACVM's blackbox solver.
2. **FFI Bindings**: Through FFI, the project binds to the Barretenberg library, ensuring a seamless and efficient interaction.
3. **ACVM Integration**: The ACVM package is an integral part of `noir_rs`, facilitating circuit execution and witness solving, simplifying zkSNARK proof generation.

## Future Work:

**Swift Bridge for Darwin-based Systems**: One of the prospective enhancements for `noir_rs` is the creation of a Swift bridge. This would empower developers to generate and verify zkSNARK proofs directly within their applications, extending support to macOS, iOS, and other Darwin-based systems.