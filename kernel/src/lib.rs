#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(type_alias_impl_trait)]

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
