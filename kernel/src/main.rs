#![no_std]
#![no_main]

use core::arch::global_asm;

use crate::sbi::shutdown;

mod console;
mod sbi;

const PROMPT: &str = "[hina]: ";

global_asm!(include_str!("entry.s"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(_hart_id: usize, _dtb: usize) -> ! {
    println!("FIREEEEEEEEEEEE");
    shutdown();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("DON'T PANIC");
    shutdown();
}
