# better_mimalloc_rs

A drop-in global allocator wrapper around the [mimalloc](https://github.com/acking-you/mimalloc) allocator.
This fork exposes tuning knobs for RSS behavior and tracks the latest mimalloc **v2** release.

## Why "better"

- **More tunable**: upstream Rust bindings expose very few knobs. This crate exposes compile-time and runtime options
  so you can tune purging/commit behavior and make RSS fall back faster.
- **Tracks v2 releases**: the vendored C sources follow the newest mimalloc v2 release (the line the tuning
  options above are defined for), merged into the `acking-you/mimalloc` fork.

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

The `config` feature implies `extended`, which also provides `MiMalloc::stats_json()` for
process-wide statistics as JSON.

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

## Usage with the `Allocator` API

The `nightly_allocator_api` feature implements `core::alloc::Allocator` for `MiMalloc`, so it can back
individual collections. It needs Rust 1.100 or newer, where the trait is stable; the feature name is kept
from upstream `mimalloc`.

```ini
[dependencies]
better_mimalloc_rs = { version = "*", features = ["nightly_allocator_api"] }
```
