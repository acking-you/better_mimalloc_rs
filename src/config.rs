//! Runtime/compile-time configuration for mimalloc options.

use crate::ffi;

include!(concat!(env!("OUT_DIR"), "/mimalloc_config.rs"));

/// Configuration for mimalloc runtime options.
///
/// Call [`apply`] before the first allocation for deterministic behavior.
#[derive(Clone, Copy, Debug, Default)]
pub struct MiMallocConfig {
    pub eager_commit: Option<bool>,
    pub eager_commit_delay: Option<i64>,
    pub arena_eager_commit: Option<i64>,
    pub purge_decommits: Option<bool>,
    pub purge_delay: Option<i64>,
    pub arena_purge_mult: Option<i64>,
    pub purge_extend_delay: Option<i64>,
    pub generic_collect: Option<i64>,
}

impl MiMallocConfig {
    /// Build a config populated from compile-time environment variables.
    pub const fn from_build() -> Self {
        Self {
            eager_commit: BUILD_EAGER_COMMIT,
            eager_commit_delay: BUILD_EAGER_COMMIT_DELAY,
            arena_eager_commit: BUILD_ARENA_EAGER_COMMIT,
            purge_decommits: BUILD_PURGE_DECOMMITS,
            purge_delay: BUILD_PURGE_DELAY,
            arena_purge_mult: BUILD_ARENA_PURGE_MULT,
            purge_extend_delay: BUILD_PURGE_EXTEND_DELAY,
            generic_collect: BUILD_GENERIC_COLLECT,
        }
    }

    /// Apply configuration to mimalloc options.
    pub fn apply(&self) {
        unsafe {
            if let Some(value) = self.eager_commit {
                ffi::mi_option_set_enabled(ffi::mi_option_eager_commit, value);
            }
            if let Some(value) = self.eager_commit_delay {
                ffi::mi_option_set(ffi::mi_option_eager_commit_delay, value as _);
            }
            if let Some(value) = self.arena_eager_commit {
                ffi::mi_option_set(ffi::mi_option_arena_eager_commit, value as _);
            }
            if let Some(value) = self.purge_decommits {
                ffi::mi_option_set_enabled(ffi::mi_option_purge_decommits, value);
            }
            if let Some(value) = self.purge_delay {
                ffi::mi_option_set(ffi::mi_option_purge_delay, value as _);
            }
            if let Some(value) = self.arena_purge_mult {
                ffi::mi_option_set(ffi::mi_option_arena_purge_mult, value as _);
            }
            if let Some(value) = self.purge_extend_delay {
                ffi::mi_option_set(ffi::mi_option_purge_extend_delay, value as _);
            }
            if let Some(value) = self.generic_collect {
                ffi::mi_option_set(ffi::mi_option_generic_collect, value as _);
            }
        }
    }
}

/// Apply the compile-time configuration defaults.
#[inline]
pub fn apply_build() {
    MiMallocConfig::from_build().apply();
}
