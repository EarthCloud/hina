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

impl Default for PlatformInfo {
    fn default() -> Self {
        Self {
            memory: [MemoryRegion::default(); MAX_MEMORY_REGIONS],
            memory_len: 0,
            reserved: [MemoryRegion::default(); MAX_RESERVED_REGIONS],
            reserved_len: 0,
            cpus: [CpuInfo::default(); MAX_CPUS],
            cpu_len: 0,
            timebase_frequency: 0,
            uart: None,
            plic: None,
            virtio_mmio: [VirtioMmioInfo::default(); MAX_VIRTIO_MMIO],
            virtio_len: 0,
            initrd: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PlatformError {
    BadFdt,
    MissingCpus,
    MissingMemory,
    MissingTimebase,
    MissingUartReg,
    MissingUartIrq,
    MissingPlicReg,
    MissingVirtioReg,
    MissingVirtioIrq,
    TooManyMemoryRegions,
    TooManyReservedRegions,
    TooManyCpus,
    TooManyVirtioMmio,
}
