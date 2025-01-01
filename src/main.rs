#![no_std]
#![no_main]
// Replace default test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// The custom test framework generates `main` that calls `test_runner`, but we are no_main
// We gotta change the name of generated function
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga;

/// Function to call on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

/// Rust by default mangles with names, we want a _start function
/// Also we want 'C' calling convention as Rust has an unspecified calling convention.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32)
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

