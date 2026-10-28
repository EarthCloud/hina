//! A very thin formatting layer on top of SBI console output.
//! No locks yet, so this is not safe for SMP output.

use core::fmt::{self, Write};

use crate::sbi::console_write;

struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        console_write(s.as_bytes());
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments<'_>) {
    let _ = Console.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::console::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::print!("\n");
    }};
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*));
    }};
}
