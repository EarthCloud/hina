use core::sync::atomic::{AtomicUsize, Ordering};

use kernel::platform::qemu_virt::TIMEBASE_FREQUENCY;

use crate::csr::{SIE_STIE, SSTATUS_SIE, r_sie, r_sstatus, w_sie, w_sstatus};
use crate::println;
use crate::{csr::r_time, sbi};

const TIMER_HZ: u64 = 10;

static TICKS: AtomicUsize = AtomicUsize::new(0);

pub fn now() -> u64 {
    r_time() as u64
}

pub fn interval() -> u64 {
    TIMEBASE_FREQUENCY / TIMER_HZ
}

pub fn set_next() {
    let next = now() + interval();
    let err = sbi::set_timer(next);
    if err != 0 {
        println!("[hina] SBI set_timer failed: err={err}");
        sbi::shutdown();
    }
}

pub fn tick() {
    TICKS.fetch_add(1, Ordering::Relaxed);
}

pub fn get_ticks() -> usize {
    TICKS.load(Ordering::Relaxed)
}

/// Initialize timer, turn on clock interrupt
pub fn init() {
    let sie = r_sie();
    w_sie(sie | SIE_STIE);

    let sstatus = r_sstatus();
    w_sstatus(sstatus | SSTATUS_SIE);

    set_next();
}
