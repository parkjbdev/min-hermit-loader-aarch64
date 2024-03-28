#![no_std]
#![no_main]

#[macro_use]
mod macros;

mod allocator;
mod arch;
mod console;
mod log;
mod none;

#[cfg(target_os = "none")]
#[doc(hidden)]
fn _print(args: core::fmt::Arguments<'_>) {
    use core::fmt::Write;

    unsafe {
        console::CONSOLE.write_fmt(args).unwrap();
    }
}
