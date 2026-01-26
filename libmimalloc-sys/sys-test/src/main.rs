#![allow(bad_style, unused_imports, unused_macros, clippy::all)]

use better_mimalloc_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
