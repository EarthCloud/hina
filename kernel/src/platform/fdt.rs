//! I wrote a simple function to parse,
//! thank god, we have some dependency to take on.

use fdt::{
    FdtError,
    nodes::{Node, root::Root},
    parsing::{NoPanic, unaligned::UnalignedParser},
    properties::interrupts::LegacyInterrupts,
};

use crate::platform::info::{
    CpuInfo, MAX_CPUS, MAX_MEMORY_REGIONS, MAX_RESERVED_REGIONS, MAX_VIRTIO_MMIO, MemoryRegion,
    PlatformError, PlatformInfo, PlicInfo, UartInfo, VirtioMmioInfo,
};

// The crate is generic over parser mode. These aliases keep things readable.
type Fdt<'a> = fdt::Fdt<'a, (UnalignedParser<'a>, NoPanic)>;
type FdtNode<'a> = Node<'a, (UnalignedParser<'a>, NoPanic)>;
type FdtRoot<'a> = Root<'a, (UnalignedParser<'a>, NoPanic)>;
type FdtInterrupts<'a> = LegacyInterrupts<'a, (UnalignedParser<'a>, NoPanic)>;

// Any parse failure means the DTB is not usable here.
impl From<FdtError> for PlatformError {
    fn from(_: FdtError) -> Self {
        Self::BadFdt
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn fdt_parse(dtb: *const u8) -> Result<PlatformInfo, PlatformError> {
    // Just collect resources. Drivers come later.
    let fdt: Fdt<'_> = unsafe { fdt::Fdt::from_ptr_unaligned_fallible(dtb) }?;
    let header = fdt.header();
    let mut info = PlatformInfo::default();

    // The crate does not expose the reservation map, so walk it by hand.
    // Entries are big-endian (start, size) pairs, terminated by zeroes.
    let bytes = unsafe { core::slice::from_raw_parts(dtb, header.total_size as usize) };
    let map = bytes
        .get(header.memory_reserve_map_offset as usize..)
        .ok_or(PlatformError::BadFdt)?;
    for entry in map.chunks_exact(16) {
        let start = u64::from_be_bytes(entry[..8].try_into().unwrap());
        let size = u64::from_be_bytes(entry[8..].try_into().unwrap());
        if start == 0 && size == 0 {
            break;
        }
        if info.reserved_len == MAX_RESERVED_REGIONS {
            return Err(PlatformError::TooManyReservedRegions);
        }
        info.reserved[info.reserved_len] = MemoryRegion { start, size };
        info.reserved_len += 1;
    }

    let root = fdt.root()?;
    // One /memory node may describe several ranges in reg.
    for entry in root.memory()?.reg()?.iter::<u64, u64>() {
        let entry = entry.map_err(|_| PlatformError::BadFdt)?;
        if info.memory_len == MAX_MEMORY_REGIONS {
            return Err(PlatformError::TooManyMemoryRegions);
        }
        info.memory[info.memory_len] = MemoryRegion {
            start: entry.address,
            size: entry.len,
        };
        info.memory_len += 1;
    }
    if info.memory_len == 0 {
        return Err(PlatformError::MissingMemory);
    }

    // QEMU virt calls the UART ns16550a.
    let uart = find(root, &["ns16550a", "ns16550"]).ok_or(PlatformError::MissingUartReg)?;
    let (base, size) = reg(&uart).ok_or(PlatformError::MissingUartReg)?;
    let uart_irq = irq(&uart).ok_or(PlatformError::MissingUartIrq)?;
    info.uart = Some(UartInfo {
        base,
        size,
        irq: uart_irq,
    });

    // For now we only need the PLIC MMIO window.
    let plic =
        find(root, &["sifive,plic-1.0.0", "riscv,plic0"]).ok_or(PlatformError::MissingPlicReg)?;
    let (base, size) = reg(&plic).ok_or(PlatformError::MissingPlicReg)?;
    info.plic = Some(PlicInfo { base, size });

    // Each virtio-mmio slot is a separate node.
    for node in root.all_compatible(&["virtio,mmio"])? {
        let node = node?;
        let (base, size) = reg(&node).ok_or(PlatformError::MissingVirtioReg)?;
        let irq = irq(&node).ok_or(PlatformError::MissingVirtioIrq)?;
        if info.virtio_len == MAX_VIRTIO_MMIO {
            return Err(PlatformError::TooManyVirtioMmio);
        }
        info.virtio_mmio[info.virtio_len] = VirtioMmioInfo { base, size, irq };
        info.virtio_len += 1;
    }

    // The timebase is shared on /cpus, hartid lives in each cpu reg.
    let cpus = root.cpus().map_err(|_| PlatformError::MissingCpus)?;
    info.timebase_frequency = cpus
        .common_timebase_frequency()?
        .ok_or(PlatformError::MissingTimebase)?;
    for cpu in cpus.iter()? {
        let hartid = cpu?
            .clone()
            .reg::<u32>()?
            .first()
            .map_err(|_| PlatformError::BadFdt)?;
        if info.cpu_len == MAX_CPUS {
            return Err(PlatformError::TooManyCpus);
        }
        info.cpus[info.cpu_len] = CpuInfo { hartid };
        info.cpu_len += 1;
    }
    if info.cpu_len == 0 {
        return Err(PlatformError::MissingCpus);
    }

    // Optional initrd passed in by the bootloader.
    if let Some(chosen) = root.find_node("/chosen")? {
        let start = property_u64(&chosen, "linux,initrd-start")?;
        let end = property_u64(&chosen, "linux,initrd-end")?;
        if let (Some(start), Some(end)) = (start, end) {
            info.initrd = Some(MemoryRegion {
                start,
                size: end.checked_sub(start).ok_or(PlatformError::BadFdt)?,
            });
        }
    }

    Ok(info)
}

// First node matching any of the given compatible strings.
fn find<'a>(root: FdtRoot<'a>, compatible: &[&str]) -> Option<FdtNode<'a>> {
    root.all_compatible(compatible).ok()?.next()?.ok()
}

// First reg entry. Address and size use the parent's cell sizes.
fn reg(node: &FdtNode<'_>) -> Option<(u64, u64)> {
    let reg = node.reg().ok()??;
    let entry = reg.iter::<u64, u64>().next()?.ok()?;
    Some((entry.address, entry.len))
}

// First interrupt specifier. RISC-V PLIC uses one cell.
fn irq(node: &FdtNode<'_>) -> Option<u32> {
    node.property::<FdtInterrupts>()
        .ok()??
        .iter()
        .next()?
        .single()
}

// Chosen properties may be encoded as 32 or 64 bit.
fn property_u64(node: &FdtNode<'_>, name: &str) -> Result<Option<u64>, PlatformError> {
    node.raw_property(name)?
        .map(|property| property.as_value().map_err(|_| PlatformError::BadFdt))
        .transpose()
}
