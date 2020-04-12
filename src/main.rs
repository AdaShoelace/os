#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use os::{
    println,
    init,
    hlt_loop,
    memory,
};

entry_point!(kernel_main);

pub  fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::registers::control::Cr3;
    use x86_64::{structures::paging::{Page, FrameAllocator}, VirtAddr};
    println!("Hello World!");

    init();

    #[cfg(test)]
    test_main();

    println!("It did not crash?!");
    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}
