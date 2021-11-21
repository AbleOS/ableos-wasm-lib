#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        super::exit();
    }
    loop {}
}

use core::panic::PanicInfo;
