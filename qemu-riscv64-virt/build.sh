cargo build --release
cp target/riscv64gc-unknown-none-elf/release/rusty-scrapyard-rv64-virt target/main.elf
llvm-objdump -d target/main.elf > target/asm.dump
