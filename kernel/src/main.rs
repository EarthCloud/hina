#![no_std]
#![no_main]

use core::arch::global_asm;

mod sbi;

global_asm!(include_str!("entry.s"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(_hart_id: usize, _dtb: usize) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
