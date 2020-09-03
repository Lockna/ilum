# ilum-init
This crate provides a startup routine for the CPU. It'll be used by the `ilum-hal` and `ilum-bsp` crate for usage.

This project might be similar to the `cortex-m-rt` crate, which is also used to
bootstrap Cortex-M systems. But since the iMXRT1062 is using a custom memory layout, which `cortex-m-rt` doesn't support, we need a custom startup crate, 
that initializes the system.