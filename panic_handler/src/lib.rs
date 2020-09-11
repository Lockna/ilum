//! Set the panicking behavior to halt
//! 
//! Usage:
//!
//! ```
//! #![no_std]
//! extern crate panic_handler;
//! 
//! fn main() {
//!		panic!("argument isn't used in any way")
//! }
//! ```

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::panic::PanicInfo;

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

	loop {}

}