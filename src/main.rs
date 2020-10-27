// disabling standard library
#![no_std]
// disabling default entry points
#![no_main]

use core::panic::PanicInfo;

// this function invoked by the compiler when a panic occurs
#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

// this attribute disables the name mangling
#[no_mangle]
// this function is the entry point, we used _start because,
// linker will look for _start function by default,
// we have used is the extern "C", this is used to tell the compiler
// to use the C calling convention for this function.
pub extern "C" fn _start() -> ! {
    // the goal is just to overwrite the entry point so we just looped indefinitely
    loop {}
}
