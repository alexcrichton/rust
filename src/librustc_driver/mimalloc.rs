#![cfg(not(bootstrap))]
use std::alloc::{GlobalAlloc, Layout};

#[cfg(all(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "asmjs",
    target_arch = "wasm32"
)))]
const MAX_ALIGN_T: usize = 8;
#[cfg(all(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "s390x",
    target_arch = "sparc64"
)))]
const MAX_ALIGN_T: usize = 16;

#[derive(Copy, Clone, Default, Debug)]
struct Mimalloc;

fn fundamental_alignment(size: usize, align: usize) -> bool {
    align <= MAX_ALIGN_T && align <= size
}

#[global_allocator]
static M: Mimalloc = Mimalloc;

unsafe impl GlobalAlloc for Mimalloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let ptr = if fundamental_alignment(size, align) {
            mimalloc_sys::mi_malloc(size as _)
        } else {
            mimalloc_sys::mi_malloc_aligned(size as _, align as _)
        };

        ptr as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let ptr = if fundamental_alignment(size, align) {
            mimalloc_sys::mi_zalloc(size as _)
        } else {
            mimalloc_sys::mi_zalloc_aligned(size as _, align as _)
        };

        ptr as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mimalloc_sys::mi_free(ptr as *mut _);
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let ptr = if fundamental_alignment(size, align) {
            mimalloc_sys::mi_realloc(ptr as *mut _, new_size)
        } else {
            mimalloc_sys::mi_realloc_aligned(ptr as *mut _, new_size, align)
        };

        ptr as *mut u8
    }
}

#[allow(dead_code)]
mod mimalloc_sys {
    //! Raw FFI wrapper over the mimalloc memory allocator
    use libc::{c_int, c_void, size_t, FILE};

    #[cfg_attr(unix, link(name = "mimalloc"))]
    #[cfg_attr(windows, link(name = "mimalloc-static"))]
    extern "C" {
        // Standard malloc interface

        pub fn mi_malloc(size: size_t) -> *mut c_void;
        pub fn mi_calloc(count: size_t, size: size_t) -> *mut c_void;
        pub fn mi_realloc(p: *mut c_void, newsize: size_t) -> *mut c_void;
        pub fn mi_expand(p: *mut c_void, newsize: size_t) -> *mut c_void;
        pub fn mi_posix_memalign(ptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
        pub fn mi_aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;
        pub fn mi_free(p: *mut c_void);
        pub fn mi_malloc_size(p: *const c_void) -> size_t;
        pub fn mi_malloc_usable_size(p: *const c_void) -> size_t;

        // Extended functionality

        pub fn mi_zalloc(size: size_t) -> *mut c_void;
        pub fn mi_usable_size(p: *const c_void) -> size_t;
        pub fn mi_good_size(size: size_t) -> size_t;

        pub fn mi_collect(force: bool);
        pub fn mi_stats_print(out: *mut FILE);
        pub fn mi_stats_reset();

        // Aligned allocation

        pub fn mi_malloc_aligned(size: size_t, alignment: size_t) -> *mut c_void;
        pub fn mi_zalloc_aligned(size: size_t, alignment: size_t) -> *mut c_void;
        pub fn mi_calloc_aligned(count: size_t, size: size_t, alignment: size_t) -> *mut c_void;
        pub fn mi_realloc_aligned(
            p: *mut c_void,
            newsize: size_t,
            alignment: size_t,
        ) -> *mut c_void;
    }
}
