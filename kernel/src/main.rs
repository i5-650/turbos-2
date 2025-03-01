#![no_std]
#![no_main]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;
use framebuffer::Color;

mod framebuffer;
mod serial;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        let info = fb.info();
        let buffer = fb.buffer_mut();
        framebuffer::init(buffer, info);

        println!("Welcome on the worst OS");
        set_color!(Color::BLUE);
        println!("I can be blue");

        set_color!(Color::GREEN);
        println!("I can be green");
    }

    serial_println!("Test");

    turbos_kernel::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop {}
}
