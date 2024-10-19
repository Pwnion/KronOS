#![no_std] // Don't link the Rust standard library.
#![no_main] // Disable Rust-level entry points.

use core::panic::PanicInfo;

use uefi::{boot, println};
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    println!("Hello World!");
    boot::stall(10_000_000);
    Status::SUCCESS
}

/// The panic handler.
/// This is required as we don't have the default panic handler from the standard library.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}