use core::fmt::Write;
use core::mem::MaybeUninit;
use core::slice;

use hermit_entry::elf::KernelObject;
use log::info;

use crate::{arch, console};

extern "C" {
    static kernel_end: u8;
    static kernel_start: u8;
}

/// Entry Point of the BIOS Loader
/// (called from entry.asm or entry.rs)
#[no_mangle]
pub(crate) unsafe extern "C" fn loader_main() -> ! {
    arch::message_output_init();
    crate::log::init();

    info!("Loader: [{:p} - {:p}]", &kernel_start, &kernel_end);

    let kernel = arch::find_kernel();
    let kernel = KernelObject::parse(kernel).unwrap();

    let mem_size = kernel.mem_size();
    let kernel_addr = unsafe { arch::get_memory(mem_size as u64) };
    let kernel_addr = kernel.start_addr().unwrap_or(kernel_addr);
    let memory = unsafe {
        slice::from_raw_parts_mut(
            sptr::from_exposed_addr_mut::<MaybeUninit<u8>>(kernel_addr as usize),
            mem_size,
        )
    };

    let kernel_info = kernel.load_kernel(memory, memory.as_ptr() as u64);

    unsafe { arch::boot_kernel(kernel_info) }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    // We can't use `println!` or related macros, because `_print` unwraps a result and might panic again
    writeln!(unsafe { &mut console::CONSOLE }, "[LOADER] {info}").ok();

    loop {}
}
