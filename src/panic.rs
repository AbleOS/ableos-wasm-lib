#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    super::exit();
    loop {}
}

use core::panic::PanicInfo;
