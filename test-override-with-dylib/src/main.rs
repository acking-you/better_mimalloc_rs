//! Test that when overriding, that the `malloc` and `free` symbols are
//! interoperable, even across a dylib boundary.
use core::ffi::{c_char, c_void, CStr};

// Make sure that `rustc` links this.
use better_mimalloc_sys as _;

extern "C-unwind" {
    fn dep_lookup_malloc_address() -> *const c_char;
    fn dep_malloc(size: libc::size_t) -> *mut c_void;
    fn dep_free(ptr: *mut c_void);
}

fn lookup_malloc_address() -> *const c_char {
    unsafe {
        let mut info: libc::Dl_info = core::mem::zeroed();
        let fnptr: unsafe extern "C" fn(libc::size_t) -> *mut c_void = libc::malloc;
        let fnptr = fnptr as *const c_void;
        if libc::dladdr(fnptr, &mut info) == 0 {
            libc::printf(b"failed finding `malloc`\n\0".as_ptr().cast());
            libc::abort();
        }
        info.dli_fname
    }
}

fn main() {
    // Check that pointers created with `malloc` in a dylib dependency can be
    // free'd with `free` here.
    let ptr = unsafe { libc::malloc(10) };
    unsafe { dep_free(ptr) };
    let ptr = unsafe { dep_malloc(10) };
    unsafe { libc::free(ptr) };

    // If overidden, test that the same is true for `mi_malloc` being
    // interoperable with `free`.
    if cfg!(feature = "override") {
        let ptr = unsafe { better_mimalloc_sys::mi_malloc(10) };
        unsafe { dep_free(ptr) };
        let ptr = unsafe { better_mimalloc_sys::mi_malloc(10) };
        unsafe { libc::free(ptr) };

        let ptr = unsafe { libc::malloc(10) };
        unsafe { better_mimalloc_sys::mi_free(ptr) };
        let ptr = unsafe { dep_malloc(10) };
        unsafe { better_mimalloc_sys::mi_free(ptr) };
    }

    // Extra check that the symbol was actually from the same place.
    let dep = unsafe { CStr::from_ptr(dep_lookup_malloc_address()) };
    let here = unsafe { CStr::from_ptr(lookup_malloc_address()) };

    if cfg!(target_vendor = "apple") {
        // macOS / Mach-O symbols are not overriden in dependencies, they are
        // hooked into with `zone_register`.
        assert_eq!(
            dep.to_str().unwrap(),
            "/usr/lib/system/libsystem_malloc.dylib"
        );
    } else {
        assert_eq!(dep, here);
    }
}
