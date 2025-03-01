#![no_std]
#![feature(abi_x86_interrupt)]

pub mod framebuffer;
pub mod interrupts;
pub mod serial;

use framebuffer::Color;
use interrupts::{gdt, idt, pics::PICS};

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init() {
    print!("GDT....");
    gdt::init();
    set_color!(Color::GREEN);
    println!("OK");
    set_color!(Color::WHITE);

    print!("IDT....");
    idt::init();
    set_color!(Color::GREEN);
    println!("OK");
    set_color!(Color::WHITE);

    print!("PICS....");
    unsafe { PICS.write().initialize() };
    set_color!(Color::GREEN);
    println!("OK");
    set_color!(Color::WHITE);

    print!("Interrupts... ");
    x86_64::instructions::interrupts::enable();
    set_color!(Color::GREEN);
    println!("OK");
    set_color!(Color::WHITE);
}
