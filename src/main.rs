#![no_std]
#![no_main]
// Replace default test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
// The custom test framework generates `main` that calls `test_runner`, but we are no_main
// We gotta change the name of generated function
#![reexport_test_harness_main = "test_main"]

mod serial;
#[cfg(test)]
mod test;
mod vga;

/// Function to call on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
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
