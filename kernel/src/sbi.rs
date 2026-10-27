//! This is a very first console solution,
//! since we have no available console.
//! Just call the OpenSBI.
//! See also: https://github.com/riscv-software-src/opensbi
use core::arch::asm;

const SBI_SYS_RESET_EXT: usize = 0x5352_5354;
const SBI_SYS_RESET_FUN: usize = 0x0;
const SBI_SYS_SHUTDOWN_TYPE: usize = 0x0;

const SBI_DBG_CONSOLE_EXT: usize = 0x4442_434E;
const SBI_DBG_CONSOLE_WRITE_FUN: usize = 0x00;
const SBI_DBG_CONSOLE_WRITE_BYTE_FUN: usize = 0x02;

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

fn console_put_byte(byte: u8) -> (isize, usize) {
    sbi_call(
        SBI_DBG_CONSOLE_EXT,
        SBI_DBG_CONSOLE_WRITE_BYTE_FUN,
        [byte as usize, 0, 0, 0, 0, 0],
    )
}

pub fn console_write(bytes: &[u8]) {
    let mut offset = 0;

    while offset < bytes.len() {
        let remaining = bytes.len() - offset;
        let addr = bytes[offset..].as_ptr() as usize;

        let (err, val) = sbi_call(
            SBI_DBG_CONSOLE_EXT,
            SBI_DBG_CONSOLE_WRITE_FUN,
            [remaining, addr, 0, 0, 0, 0],
        );

        if err != 0 {
            break;
        }

        if val == 0 {
            // Bulk write made no progress. Use the blocking byte call
            // so the kernel cannot spin forever.
            let (err, _) = console_put_byte(bytes[offset]);
            if err != 0 {
                return;
            }
            offset += 1;
        }

        offset += val.min(remaining);
    }
}

pub fn shutdown() -> ! {
    let (err, _) = sbi_call(
        SBI_SYS_RESET_EXT,
        SBI_SYS_RESET_FUN,
        [SBI_SYS_SHUTDOWN_TYPE, 0, 0, 0, 0, 0],
    );

    if err != 0 {
        console_write(b"SBI shutdown failed\n");
    }

    park();
}

pub fn park() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
