#![no_std]
#![no_main]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    turbos_kernel::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
