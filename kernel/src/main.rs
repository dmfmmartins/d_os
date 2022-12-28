#![no_std]
#![no_main]

use bootloader_api::*;
use log::*;
use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    com_logger::init();
    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);
    mode.draw_line((80, 60), (80, 420), Color16::White);
    mode.draw_line((80, 60), (540, 60), Color16::White);
    mode.draw_line((80, 420), (540, 420), Color16::White);
    mode.draw_line((540, 420), (540, 60), Color16::White);
    mode.draw_line((80, 90), (540, 90), Color16::White);
    info!("{:#?}", mode);

    loop {}
}

const CONFIG: BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};
entry_point!(kernel_main, config = &CONFIG);

#[panic_handler]
// #[cfg(not(test))]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("{:?}", info);
    loop {}
}
