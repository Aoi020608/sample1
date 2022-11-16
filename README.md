# Solana Hello World Program

## Overview
Write first solana program.

## Logging
- Use msg! macro
- [Example](https://github.com/solana-labs/solana-program-library/tree/master/examples/rust/logging/src)
- When working on a local node, using the CLI tool ~solana-test-validator~, via a client with RPC calls on [Rust](https://docs.rs/solana-client/latest/solana_client/).

## Errors
- [Error Handling](https://docs.solana.com/developing/on-chain-programs/debugging#error-handling)
- The thiserror crate comes in handy. [example](https://github.com/metaplex-foundation/token-vesting/tree/master/program/src)

## Getting Started
1. Run
```bash
cargo init --lib solana-hello-world && cd solana-hello-world
```
The program is not an executable. You need to start with lib.rs.

## References
- [solana-hello-world](https://github.com/siexp/solana-hello-world/blob/master/tests/lib.rs)

