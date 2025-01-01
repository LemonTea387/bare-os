#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;

/// Function to call on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Rust by default mangles with names, we want a _start function
/// Also we want 'C' calling convention as Rust has an unspecified calling convention.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    {
        let mut writer = vga::VGA_Writer.lock();
        let _ = writeln!(writer, "This is big pog {}", 1337);
    }
    loop {}
}
