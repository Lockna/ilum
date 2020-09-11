#![no_std]

mod cache;
mod fault;
mod fpu;
mod nvic;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
	cache::init();
	fpu::init();
	nvic::init();

	extern "Rust" {
        fn main() -> !;
    }

    #[inline(never)]
    fn trampoline() -> ! {
        unsafe { main() };
    }

    trampoline();

}