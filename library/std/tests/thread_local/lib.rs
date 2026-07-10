#![feature(cfg_target_thread_local)]

#[cfg(not(target_os = "emscripten"))]
mod tests;

mod dynamic_tests;
