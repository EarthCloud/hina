//! Hardcoded QEMU virt board information.
//! This is enough for M0. Device tree can replace this table later.

pub const MEMORY_START: u64 = 0x8000_0000;
pub const MEMORY_SIZE: u64 = 128 << 20;
pub const MEMORY_END: u64 = MEMORY_START + MEMORY_SIZE;

pub const CPUS: [u32; 4] = [0, 1, 2, 3];
pub const TIMEBASE_FREQUENCY: u64 = 10_000_000;

pub const UART_BASE: u64 = 0x1000_0000;
pub const UART_SIZE: u64 = 0x100;
pub const UART_IRQ: u32 = 10;

pub const PLIC_BASE: u64 = 0x0c00_0000;
pub const PLIC_SIZE: u64 = 0x60_0000;

pub const VIRTIO_BASE: u64 = 0x1000_1000;
pub const VIRTIO_SIZE: u64 = 0x1000;
pub const VIRTIO_COUNT: usize = 8;
