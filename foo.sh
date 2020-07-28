set -ex

# ./configure \
#   --set rust.deny-warnings=false \
#   --set target.wasm32-wasi.wasi-root=/home/alex/code/wasi-libc/sysroot \
#   --set rust.lld \
#   --enable-ccache

# ./x.py build --target wasm32-wasi --stage 1 library/std

# rm -rf rust-sysroot
# mkdir rust-sysroot
# cp -r build/x86_64-unknown-linux-gnu/stage1/lib rust-sysroot/lib

# ./configure \
#   --set rust.deny-warnings=false \
#   --set target.wasm32-wasi.wasi-root=/home/alex/code/wasi-libc/sysroot \
#   --set rust.codegen-backends=[]

# RUSTFLAGS='-Ctarget-feature=+bulk-memory,+simd128' \
# PATH=`pwd`/rust-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/bin:$PATH \
#   ./x.py build --host wasm32-wasi --stage 1 src/rustc

wasmtime="../wasmtime/target/release/wasmtime \
    run \
    --enable-simd \
    --jitdump \
    --enable-bulk-memory \
    --mapdir /::. \
    --mapdir /sysroot::./rust-sysroot \
    --"

rustc="$wasmtime \
  build/wasm32-wasi/stage1/bin/rustc.wasm \
    --sysroot /sysroot \
    -Z codegen-backend=dummy"

echo 'pub fn foo() {}' > foo.rs
echo 'pub fn bar() { foo::foo(); }' > bar.rs

$rustc \
    /foo.rs \
    --emit metadata \
    -o /libfoo.rmeta \
    --crate-type lib

perf record -k 1 $rustc \
    /bar.rs \
    --emit metadata \
    -o /libbar.rmeta \
    --crate-type lib \
    --extern foo=/libfoo.rmeta
