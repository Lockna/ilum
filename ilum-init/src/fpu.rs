use core::ptr;

#[inline(always)]
pub unsafe fn init() {

    const SCB_CPACR: *mut u32 = 0xE000_ED88 as *mut u32;
    const SCB_CPACR_FPU_ENABLE: u32 = 0x00F00000;

    ptr::write_volatile(SCB_CPACR, SCB_CPACR_FPU_ENABLE);


}