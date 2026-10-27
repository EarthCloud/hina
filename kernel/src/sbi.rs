/// See also: https://github.com/riscv-software-src/opensbi
use core::arch::asm;

const SBI_SYS_RESET_EXT: usize = 0x5352_5354;
const SBI_SYS_RESET_FUN: usize = 0x0;
const SBI_SYS_SHUTDOWN_TYPE: usize = 0x0;


#[inline(always)]
fn sbi_call(eid: usize, fid: usize, args: [usize; 6]) -> (isize, usize) {
    let [a0, a1, a2, a3, a4, a5] = args;

    let err: usize;
    let val: usize;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") a0 => err,
            inlateout("a1") a1 => val,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a5") a5,
            in("a6") fid,
            in("a7") eid,
            options(nostack),
        );
    }

    (err as isize, val)
}
