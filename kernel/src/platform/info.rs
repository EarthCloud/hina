//! All the things we need to show when we start
pub const MAX_MEMORY_REGIONS: usize = 4;
pub const MAX_RESERVED_REGIONS: usize = 8;
pub const MAX_CPUS: usize = 8;
pub const MAX_VIRTIO_MMIO: usize = 8;

#[derive(Clone, Copy, Debug, Default)]
pub struct MemoryRegion {
    pub start: u64,
    pub size: u64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CpuInfo {
    pub hartid: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UartInfo {
    pub base: u64,
    pub size: u64,
    pub irq: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PlicInfo {
    pub base: u64,
    pub size: u64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct VirtioMmioInfo {
    pub base: u64,
    pub size: u64,
    pub irq: u32,
}

#[derive(Debug)]
pub struct PlatformInfo {
    pub memory: [MemoryRegion; MAX_MEMORY_REGIONS],
    pub memory_len: usize,

    pub reserved: [MemoryRegion; MAX_RESERVED_REGIONS],
    pub reserved_len: usize,

    pub cpus: [CpuInfo; MAX_CPUS],
    pub cpu_len: usize,
    pub timebase_frequency: u64,

    pub uart: Option<UartInfo>,
    pub plic: Option<PlicInfo>,

    pub virtio_mmio: [VirtioMmioInfo; MAX_VIRTIO_MMIO],
    pub virtio_len: usize,

    pub initrd: Option<MemoryRegion>,
}
