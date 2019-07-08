// main.rs

#![no_std]
#![no_main]

//
// Replace Panic Implementation (RIP std)
//
use core::panic::PanicInfo;

//
// Call this on panic:
//
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
   loop {}
}

//
// Overwrite program entrypoint
//
#[no_mangle]
pub extern "C" fn _start() -> ! {
   loop {}
}
