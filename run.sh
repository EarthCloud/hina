qemu-system-riscv64 \
  -machine virt \
  -m 128M \
  -smp 1 \
  -bios default \
  -kernel target/riscv64gc-unknown-none-elf/debug/hina \
  -nographic