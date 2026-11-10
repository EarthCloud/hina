//! Trap entry initialization and the minimal L1a diagnostic handler.

use crate::csr::w_stvec;
use crate::println;
use crate::sbi;

/// Very first TrapFrame, aligned with 16-bits
#[repr(C)]
pub struct TrapFrame {
    pub x: [usize; 32],
    pub sepc: usize,
    pub sstatus: usize,
    pub scause: usize,
    pub stval: usize,
}

unsafe extern "C" {
    fn trap_entry();
}

pub fn init() {
    let entry = trap_entry as *const () as usize;
    // In trap.s, trap_entry was aligned with 4 bytes
    // The low 2 bit is mode, direct mode corresponding to 00
    let entry = entry & !0b11;
    w_stvec(entry);
}

#[unsafe(no_mangle)]
pub extern "C" fn trap_panic(scause: usize, sepc: usize, stval: usize, hartid: usize) -> ! {
    println!("[hina] trap");
    println!("scause={scause:#x}");
    println!("sepc={sepc:#x}");
    println!("stval={stval:#x}");
    println!("hartid={hartid}");
    sbi::shutdown()
}
