#![no_std]
#![no_main]

use core::arch::global_asm;

use crate::sbi::{console_write, shutdown};

mod sbi;

global_asm!(include_str!("entry.s"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(_hart_id: usize, _dtb: usize) -> ! {
    console_write(b"FIREEEEEEEEEEEE\n");
    shutdown();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    console_write(b"DON'T PANIC");
    shutdown();
}
