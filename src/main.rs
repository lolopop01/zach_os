#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zach_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use zach_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    zach_os::init(); 

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3(); 

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zach_os::test_panic_handler(info)
}