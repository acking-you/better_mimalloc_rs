# better_mimalloc_rs

A drop-in global allocator wrapper around the [mimalloc](https://github.com/acking-you/mimalloc) allocator.
This fork exposes tuning knobs for RSS behavior and tracks the mimalloc **dev branch** (more aggressive than release).

## Why "better"

- **More tunable**: upstream Rust bindings expose very few knobs. This crate exposes compile-time and runtime options
  so you can tune purging/commit behavior and make RSS fall back faster.
- **Tracks dev branch**: follows the latest mimalloc development branch for newer fixes and behavior (expect churn).

## Usage

```rust
use better_mimalloc_rs::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

## RSS tuning (config feature)

Enable the `config` feature to expose runtime/compile-time tuning:

```ini
[dependencies]
better_mimalloc_rs = { version = "*", features = ["config"] }
```

Apply compile-time defaults:

```rust
use better_mimalloc_rs::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    MiMalloc::init();
    // ...
}
```

Apply runtime overrides:

```rust
use better_mimalloc_rs::{MiMalloc, MiMallocConfig};

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

Call `MiMalloc::init()`/`init_with()` before any allocations for deterministic behavior.

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

## Using the forked mimalloc source

This repo expects the C sources under `libmimalloc-sys/c_src/mimalloc`.
Initialize the submodule after cloning:

```bash
git submodule update --init --recursive
```

If you keep the sources elsewhere, set `MIMALLOC_SRC` to the root of that checkout:

```bash
export MIMALLOC_SRC=/path/to/acking-you/mimalloc
```

## Requirements

A __C__ compiler is required for building [mimalloc](https://github.com/acking-you/mimalloc) with cargo.

## Usage with secure mode

Using secure mode adds guard pages, randomized allocation, encrypted free lists, etc.
The performance penalty is usually around 10% according to mimalloc's own benchmarks.

To enable secure mode, put in `Cargo.toml`:

```ini
[dependencies]
better_mimalloc_rs = { version = "*", features = ["secure"] }
```
