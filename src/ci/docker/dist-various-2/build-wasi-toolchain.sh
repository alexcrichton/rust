#!/bin/sh

set -ex

curl https://prereleases.llvm.org/8.0.0/rc3/clang+llvm-8.0.0-rc3-x86_64-linux-gnu-ubuntu-14.04.tar.xz | \
  tar xJf -
export PATH=`pwd`/clang+llvm-8.0.0-rc3-x86_64-linux-gnu-ubuntu-14.04/bin:$PATH

# FIXME: uncomment this line and remove the next
#git clone https://github.com/cranestation/reference-sysroot-wasi
git clone /tmp/wut reference-sysroot-wasi

cd reference-sysroot-wasi
git reset --hard d0d8bc47946948646cb50a37471d9516c90d9786
make -j$(nproc) INSTALL_DIR=/wasm32-unknown-wasi
make install INSTALL_DIR=/wasm32-unknown-wasi

cd ..
rm -rf reference-sysroot-wasi
rm -rf clang+llvm*
