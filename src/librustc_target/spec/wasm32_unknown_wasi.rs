use super::{LinkerFlavor, Target};
use super::wasm32_base;

pub fn target() -> Result<Target, String> {
    let mut options = wasm32_base::options();
    options.linker = Some("clang".to_string());
    options
        .pre_link_args
        .entry(LinkerFlavor::Gcc)
        .or_insert(Vec::new())
        .push("--target=wasm32-unknown-wasi".to_string());
    Ok(Target {
        llvm_target: "wasm32-unknown-wasi".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        target_os: "unknown".to_string(),
        target_env: "wasi".to_string(),
        target_vendor: "unknown".to_string(),
        data_layout: "e-m:e-p:32:32-i64:64-n32:64-S128".to_string(),
        arch: "wasm32".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options,
    })
}
