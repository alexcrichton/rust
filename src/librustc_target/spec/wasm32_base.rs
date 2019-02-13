use std::collections::BTreeMap;
use super::{LldFlavor, TargetOptions, PanicStrategy, LinkerFlavor};

pub fn options() -> TargetOptions {
    let mut lld_args = Vec::new();
    let mut clang_args = Vec::new();
    let mut arg = |arg: &str| {
        lld_args.push(arg.to_string());
        clang_args.push(format!("-Wl,{}", arg));
    };

    // There have been reports in the wild (rustwasm/wasm-bindgen#119) of
    // using threads causing weird hangs and bugs. Disable it entirely as
    // this isn't yet the bottleneck of compilation at all anyway.
    arg("--no-threads");

    // By default LLD only gives us one page of stack (64k) which is a
    // little small. Default to a larger stack closer to other PC platforms
    // (1MB) and users can always inject their own link-args to override this.
    arg("-z");
    arg("stack-size=1048576");

    // By default LLD's memory layout is:
    //
    // 1. First, a blank page
    // 2. Next, all static data
    // 3. Finally, the main stack (which grows down)
    //
    // This has the unfortunate consequence that on stack overflows you
    // corrupt static data and can cause some exceedingly weird bugs. To
    // help detect this a little sooner we instead request that the stack is
    // placed before static data.
    //
    // This means that we'll generate slightly larger binaries as references
    // to static data will take more bytes in the ULEB128 encoding, but
    // stack overflow will be guaranteed to trap as it underflows instead of
    // corrupting static data.
    arg("--stack-first");

    // FIXME we probably shouldn't pass this but instead pass an explicit
    // whitelist of symbols we'll allow to be undefined. Unfortunately
    // though we can't handle symbols like `log10` that LLVM injects at a
    // super late date without actually parsing object files. For now let's
    // stick to this and hopefully fix it before stabilization happens.
    arg("--allow-undefined");

    // Rust code should never have warnings, and warnings are often
    // indicative of bugs, let's prevent them.
    arg("--fatal-warnings");

    // LLD only implements C++-like demangling, which doesn't match our own
    // mangling scheme. Tell LLD to not demangle anything and leave it up to
    // us to demangle these symbols later.
    arg("--no-demangle");

    // The symbol visibility story is a bit in flux right now with LLD.
    // It's... not entirely clear to me what's going on, but this looks to
    // make everything work when `export_symbols` isn't otherwise called for
    // things like executables.
    arg("--export-dynamic");

    let mut pre_link_args = BTreeMap::new();
    pre_link_args.insert(LinkerFlavor::Lld(LldFlavor::Wasm), lld_args);
    pre_link_args.insert(LinkerFlavor::Gcc, clang_args);

    TargetOptions {
        // we allow dynamic linking, but only cdylibs. Basically we allow a
        // final library artifact that exports some symbols (a wasm module) but
        // we don't allow intermediate `dylib` crate types
        dynamic_linking: true,
        only_cdylib: true,

        // This means we'll just embed a `start` function in the wasm module
        executables: true,

        // relatively self-explanatory!
        exe_suffix: ".wasm".to_string(),
        dll_prefix: String::new(),
        dll_suffix: ".wasm".to_string(),
        linker_is_gnu: false,

        max_atomic_width: Some(64),

        // Unwinding doesn't work right now, so the whole target unconditionally
        // defaults to panic=abort. Note that this is guaranteed to change in
        // the future once unwinding is implemented. Don't rely on this.
        panic_strategy: PanicStrategy::Abort,

        // Wasm doesn't have atomics yet, so tell LLVM that we're in a single
        // threaded model which will legalize atomics to normal operations.
        singlethread: true,

        // no dynamic linking, no need for default visibility!
        default_hidden_visibility: true,

        // we use the LLD shipped with the Rust toolchain by default
        linker: Some("rust-lld".to_owned()),
        lld_flavor: LldFlavor::Wasm,

        // No need for indirection here, simd types can always be passed by
        // value as the whole module either has simd or not, which is different
        // from x86 (for example) where programs can have functions that don't
        // enable simd features.
        simd_types_indirect: false,

        pre_link_args,

        .. Default::default()
    }
}
