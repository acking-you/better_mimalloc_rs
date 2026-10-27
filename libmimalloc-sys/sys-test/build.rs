use std::env;
use std::path::Path;
use std::path::PathBuf;

fn resolve_mimalloc_include() -> PathBuf {
    if let Ok(root) = env::var("MIMALLOC_SRC") {
        return PathBuf::from(root).join("include");
    }

    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_dir = Path::new(cargo_manifest_dir);

    let flat_root = manifest_dir.join("../c_src/mimalloc/include");
    if flat_root.is_dir() {
        return flat_root;
    }

    panic!(
        "mimalloc source not found. Set MIMALLOC_SRC or run `git submodule update --init --recursive` to populate c_src/mimalloc (https://github.com/acking-you/mimalloc)."
    );
}

fn main() {
    println!("cargo:rerun-if-env-changed=MIMALLOC_SRC");

    let mut cfg = ctest::TestGenerator::new();
    cfg.header("mimalloc.h")
        .header("mimalloc-stats.h")
        .include(resolve_mimalloc_include())
        // Ignore whether or not the option enum is signed.
        .skip_signededness(|ty| ty.ends_with("_t") || ty.ends_with("_e"))
        // `mi_heap_area_t` is a typedef of `struct mi_heap_area_s`, so name it directly.
        .rename_struct_ty(|ty| ty.ends_with("_t").then(|| ty.to_string()))
        // mimalloc declares its callbacks as function types (`typedef ret mi_some_fun(...)`)
        // and takes `mi_some_fun*` arguments, whereas Rust models them as function pointers.
        .rename_type(|ty| ty.ends_with("_fun").then(|| format!("{ty}*")));
    for feature in ["secure", "extended"] {
        let var = format!("CARGO_FEATURE_{}", feature.to_ascii_uppercase());
        if env::var_os(var).is_some() {
            cfg.cfg("feature", Some(feature));
        }
    }

    ctest::generate_test(&mut cfg, "../src/lib.rs", "all.rs").expect("failed to generate tests");
}
