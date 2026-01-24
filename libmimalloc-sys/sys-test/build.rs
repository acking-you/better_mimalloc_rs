use std::env;
use std::path::Path;
use std::path::PathBuf;

fn resolve_mimalloc_include(version: &str) -> PathBuf {
    if let Ok(root) = env::var("MIMALLOC_SRC") {
        return PathBuf::from(root).join("include");
    }

    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_dir = Path::new(cargo_manifest_dir);

    let versioned_root = manifest_dir.join(format!("../c_src/mimalloc/{version}/include"));
    if versioned_root.is_dir() {
        return versioned_root;
    }

    let flat_root = manifest_dir.join("../c_src/mimalloc/include");
    if flat_root.is_dir() {
        return flat_root;
    }

    panic!(
        "mimalloc source not found. Set MIMALLOC_SRC or clone https://github.com/acking-you/mimalloc into c_src/mimalloc"
    );
}

fn main() {
    println!("cargo:rerun-if-env-changed=MIMALLOC_SRC");
    let secure = if env::var("CARGO_FEATURE_SECURE").is_ok() {
        Some("secure")
    } else {
        None
    };
    let extended = if env::var("CARGO_FEATURE_EXTENDED").is_ok() {
        Some("extended")
    } else {
        None
    };
    let version = if env::var("CARGO_FEATURE_V3").is_ok() {
        "v3"
    } else {
        "v2"
    };

    let mut cfg = ctest2::TestGenerator::new();
    let include_dir = resolve_mimalloc_include(version);

    cfg.header("mimalloc.h")
        .include(include_dir)
        .cfg("feature", secure)
        .cfg("feature", extended)
        .cfg("feature", (version == "v3").then_some("v3"))
        .fn_cname(|rust, link_name| link_name.unwrap_or(rust).to_string())
        // ignore whether or not the option enum is signed.
        .skip_signededness(|c| c.ends_with("_t") || c.ends_with("_e"))
        .type_name(|ty, _is_struct, _is_union| {
            match ty {
                // Special cases. We do this to avoid having both
                // `mi_blah_{s,e}` and `mi_blah_t`.
                "mi_heap_area_t" => "struct mi_heap_area_s".into(),
                "mi_heap_t" => "struct mi_heap_s".into(),
                "mi_options_t" => "enum mi_options_e".into(),

                // This also works but requires we export `mi_heap_s` and similar
                // in addition, so we just hardcode the above.

                // t if t.ends_with("_s") => format!("struct {}", t),
                // t if t.ends_with("_e") => format!("enum {}", t),
                // t if t.ends_with("_t") => t.to_string(),

                // mimalloc defines it's callbacks with the pointer at the
                // location of use, e.g. `typedef ret mi_some_fun(a0 x, a1 y);`
                // and then uses `mi_some_fun *arg` as argument types, which
                // appears to upset ctest, which would prefer function pointers
                // be declared as pointers, so we clean things up for it.
                t if t.ends_with("_fun") => format!("{}*", t),

                t => t.to_string(),
            }
        });

    cfg.generate("../src/lib.rs", "all.rs");
}
