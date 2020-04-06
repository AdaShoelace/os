#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::{println, init};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!\n");
    
    init();
    
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    #[cfg(test)]
    test_main();

    println!("It did not crash?!\n");
    

    loop{}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}
