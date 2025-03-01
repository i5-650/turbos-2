#![no_std]
#![no_main]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;

use turbos_kernel::framebuffer;
use turbos_kernel::framebuffer::Color;
use turbos_kernel::println;
use turbos_kernel::serial_println;
use turbos_kernel::set_color;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let fb = boot_info
        .framebuffer
        .as_mut()
        .expect("framebuffer should always be present");
    let info = fb.info();
    let buffer = fb.buffer_mut();
    framebuffer::init(buffer, info);

    println!("Welcome on the worst OS");
    set_color!(Color::BLUE);
    println!("I can be blue");

    set_color!(Color::GREEN);
    println!("I can be green");
    set_color!(Color::WHITE);

    serial_println!("Before init");

    turbos_kernel::init();

    turbos_kernel::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop {}
}
