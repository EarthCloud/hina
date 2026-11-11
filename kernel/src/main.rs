#![no_std]
#![no_main]

use core::arch::global_asm;

use kernel::platform::qemu_virt as board;

use crate::sbi::shutdown;

mod console;
mod csr;
mod sbi;
mod timer;
mod trap;

const PROMPT: &str = "[hina]";

global_asm!(include_str!("entry.s"));
global_asm!(include_str!("trap.s"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(hart_id: usize, dtb: usize) -> ! {
    trap::init();
    println!("{PROMPT} M0 boot");
    println!("{PROMPT} boot hart: {hart_id}");
    println!("{PROMPT} dtb: {dtb:#018x}");

    println!(
        "{PROMPT} memory: {:#x}..{:#x} ({} MiB)",
        board::MEMORY_START,
        board::MEMORY_END,
        board::MEMORY_SIZE >> 20
    );
    println!(
        "{PROMPT} cpus: 0,1,2,3 (timebase {})",
        board::TIMEBASE_FREQUENCY
    );
    println!(
        "{PROMPT} uart: {:#x} irq {}",
        board::UART_BASE,
        board::UART_IRQ
    );
    println!("{PROMPT} plic: {:#x}", board::PLIC_BASE);
    println!("{PROMPT} virtio-mmio: {} devices", board::VIRTIO_COUNT);

    timer::init();
    
    shutdown();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("DON'T PANIC");
    shutdown();
}
