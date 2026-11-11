//! RISC-V Control and Status Registers
//! At this stage, we only use it to turn on the interrupt

#![allow(dead_code)] // TODO: remove after L1b uses the timer CSR helpers

use core::arch::asm;

// sie
pub const SIE_SSIE: usize = 1 << 1;  // supervisor software
pub const SIE_STIE: usize = 1 << 5;  // supervisor timer
pub const SIE_SEIE: usize = 1 << 9;  // supervisor external

// sstatus
pub const SSTATUS_SIE: usize = 1 << 1;   // global interrupt enable
pub const SSTATUS_SPIE: usize = 1 << 5;  // previous SIE
pub const SSTATUS_SPP: usize = 1 << 8;   // previous privilege

// sip
pub const SIP_STIP: usize = 1 << 5;      // timer pending

// scause
pub const SCAUSE_INTERRUPT: usize = 1 << 63;
pub const SCAUSE_CODE_MASK: usize = 0xff;

/// Supervisor Trap Vector Base Address Register
/// Caller must ensure `value` is 4-byte aligned and mode bits are 0.
#[inline(always)]
pub fn w_stvec(value: usize) {
    unsafe { asm!("csrw stvec, {}", in(reg) value) }
}

/// Supervisor Status Register
#[inline(always)]
pub fn w_sstatus(value: usize) {
    unsafe { asm!("csrw sstatus, {}", in(reg) value) }
}

/// Supervisor Interrupt Enable
/// Overwrites the whole CSR. For bit updates, use read-modify-write.
#[inline(always)]
pub fn w_sie(value: usize) {
    unsafe { asm!("csrw sie, {}", in(reg) value) }
}

/// Supervisor Cause Register
#[inline(always)]
pub fn r_scause() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, scause", out(reg) value) };
    value
}

/// Supervisor Exception Program Counter
#[inline(always)]
pub fn r_sepc() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, sepc", out(reg) value) };
    value
}

/// Supervisor Trap Value
#[inline(always)]
pub fn r_stval() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, stval", out(reg) value) };
    value
}

/// Supervisor Interrupt Enable
#[inline(always)]
pub fn r_sie() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, sie", out(reg) value) };
    value
}

/// Supervisor Status Register
#[inline(always)]
pub fn r_sstatus() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, sstatus", out(reg) value) };
    value
}

/// Time Register
#[inline(always)]
pub fn r_time() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, time", out(reg) value) };
    value
}
