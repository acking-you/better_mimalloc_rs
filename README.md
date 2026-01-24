# Mimalloc Rust

[![Latest Version]][crates.io] [![Documentation]][docs.rs]

A drop-in global allocator wrapper around the [mimalloc](https://github.com/microsoft/mimalloc) allocator.
Mimalloc is a general purpose, performance oriented allocator built by Microsoft.

## What is better_mimalloc_rs

This is a forked and locally vendored variant of `mimalloc_rust` used by
`proxy-everything`. It keeps the Rust API compatible while allowing us to:

- Pin the C allocator to a specific fork (`https://github.com/acking-you/mimalloc`)
- Expose allocator tuning options that affect RSS behavior

## Why fork

We need predictable, repository-controlled allocator behavior to reproduce and
adjust RSS characteristics in production. The forked setup lets us:

- Control the exact mimalloc C source version
- Apply tuning parameters at build time or runtime without patching upstream

## Usage

```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

## Using the forked mimalloc source

This repo expects the C sources under `libmimalloc-sys/c_src/mimalloc`.
Clone the fork once:

```bash
git clone https://github.com/acking-you/mimalloc.git libmimalloc-sys/c_src/mimalloc
```

If you keep the sources elsewhere, set `MIMALLOC_SRC` to the root of that checkout:

```bash
export MIMALLOC_SRC=/path/to/acking-you/mimalloc
```

## Requirements

A __C__ compiler is required for building [mimalloc](https://github.com/microsoft/mimalloc) with cargo.

## Configuring RSS-related options

Enable the `config` feature to expose runtime/compile-time tuning:

```ini
[dependencies]
mimalloc = { path = "deps/better_mimalloc_rs", features = ["config"] }
```

Apply compile-time defaults:

```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    MiMalloc::init();
    // ...
}
```

Apply runtime overrides:

```rust
use mimalloc::{MiMalloc, MiMallocConfig};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let cfg = MiMallocConfig {
        purge_delay: Some(0),
        purge_decommits: Some(true),
        ..MiMallocConfig::default()
    };
    MiMalloc::init_with(&cfg);
}
```

Call `MiMalloc::init()`/`init_with()` before any allocations for deterministic
behavior.

Compile-time environment variables (optional):

```
MIMALLOC_CFG_EAGER_COMMIT=0|1
MIMALLOC_CFG_EAGER_COMMIT_DELAY=NUM
MIMALLOC_CFG_ARENA_EAGER_COMMIT=NUM
MIMALLOC_CFG_PURGE_DECOMMITS=0|1
MIMALLOC_CFG_PURGE_DELAY=NUM
MIMALLOC_CFG_ARENA_PURGE_MULT=NUM
MIMALLOC_CFG_PURGE_EXTEND_DELAY=NUM
MIMALLOC_CFG_GENERIC_COLLECT=NUM
```

## Usage with secure mode

Using secure mode adds guard pages,
randomized allocation, encrypted free lists, etc. The performance penalty is usually
around 10% according to [mimalloc](https://github.com/microsoft/mimalloc)
own benchmarks.

To enable secure mode, put in `Cargo.toml`:

```ini
[dependencies]
mimalloc = { version = "*", features = ["secure"] }
```

## Usage with v3

By default this library uses mimalloc `v2`.
To enable `v3`, put in `Cargo.toml`:

```ini
[dependencies]
mimalloc = { version = "*", features = ["v3"] }
```

[crates.io]: https://crates.io/crates/mimalloc
[Latest Version]: https://img.shields.io/crates/v/mimalloc.svg
[Documentation]: https://docs.rs/mimalloc/badge.svg
[docs.rs]: https://docs.rs/mimalloc
