#![feature(global_asm, naked_functions)]
#![no_std]

use core::{ptr, mem};

mod cache;
mod fault;
mod fpu;
mod nvic;

global_asm!(include_str!("init.S"));

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {

    extern "C" {

        static mut __sbss: u32;
        static mut __ebss: u32;

        static mut __stext: u32;
        static mut __etext: u32;
        static __sitext: u32;

        static mut __sdata: u32;
        static mut __edata: u32;
        static __sidata: u32;

    }

    init_TCM();

    zero_bss(&mut __sbss, &mut __ebss);
    init_data(&mut __stext, &mut __etext, &__sitext);
    init_data(&mut __sdata, &mut __edata, &__sidata);

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

pub unsafe fn init_data(mut sdata: *mut u32, edata: *mut u32, mut sidata: *const u32)
{
    while sdata < edata {
        ptr::write(sdata, ptr::read(sidata));
        sdata = sdata.offset(1);
        sidata = sidata.offset(1);
    }
}

pub unsafe fn zero_bss(mut sbss: *mut u32, ebss: *mut u32)
{
    while sbss < ebss {
        ptr::write_volatile(sbss, mem::zeroed());
        sbss = sbss.offset(1);
    }
}

#[naked]
pub unsafe fn init_TCM() {

    extern "C" {
        static __flexram_bank_config: u32;
    }

    let IOMUXC_GPR = 0x400AC000 as *const u32;
    IOMUXC_GPR.offset(17) = &__flexram_bank_config;
    IOMUXC_GPR.offset(16) = 0x00200007 as *const u32;
    IOMUXC_GPR.offset(14) = 0x00AA0000 as *const u32;

}