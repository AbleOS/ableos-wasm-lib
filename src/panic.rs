#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use core::panic::PanicInfo;
