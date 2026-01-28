# RISCV64 virt machine build guide
1. Select target and linker script:
```
export CARGO_BUILD_TARGET=riscv64gc-unknown-none-elf
export CARGO_BUILD_RUSTFLAGS='-C link-arg=-Tsrc/bin/riscv64-virt/map.ld'
cargo build --release
```

2. Call `cargo build --release`

3. Get binary and run qemu sim with some devices:
```
cp target/riscv64gc-unknown-none-elf/release/riscv64-virt target/exec.elf
qemu-system-riscv64 -nographic -icount shift=10 -bios=target/exec.elf
```
