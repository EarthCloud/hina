//! RISC-V Control and Status Registers
//! At this stage, we only use it to turn on the interrupt

use core::arch::asm;

/// Supervisor Trap Vector Base Address Register
/// Caller must ensure `value` is 4-byte aligned and mode bits are 0.
#[inline(always)]
pub fn w_stvec(value: usize) {
    unsafe { asm!("csrw stvec, {}",in(reg) value) }
}

/// Supervisor Status Register
#[inline(always)]
pub fn w_sstatus(value: usize) {
    unsafe { asm!("csrw sstatus, {}",in(reg) value) }
}

/// Supervisor Interrupt Enable
/// Overwrites the whole CSR. For bit updates, use read-modify-write.
#[inline(always)]
pub fn w_sie(value: usize) {
    unsafe { asm!("csrw sie, {}",in(reg) value) }
}

/// Supervisor Cause Register
#[inline(always)]
pub fn r_scause() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, scause",out(reg) value) };
    value
}

/// Supervisor Exception Program Counter
#[inline(always)]
pub fn r_sepc() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, sepc",out(reg) value) };
    value
}

/// Supervisor Trap Value
#[inline(always)]
pub fn r_stval() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, stval",out(reg) value) };
    value
}

/// Supervisor Interrupt Enable
#[inline(always)]
pub fn r_sie() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, sie",out(reg) value) };
    value
}

/// Supervisor Status Register
#[inline(always)]
pub fn r_sstatus() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, sstatus",out(reg) value) };
    value
}

/// Time Register
#[inline(always)]
pub fn r_time() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, time",out(reg) value) };
    value
}
