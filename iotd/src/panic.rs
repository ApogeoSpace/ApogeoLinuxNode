use core::ffi::c_char;
use core::panic::PanicInfo;
use crate::log_emerg;

unsafe extern "C" {
    fn panic(fmt: *const c_char, ...) -> !;
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) ->! {
    log_emerg!("Rust driver panic'd: {}\n", info.message());
    log_emerg!("Panic report:\n");
    if let Some(location) = info.location() {
        log_emerg!("\tPanic occurred at {}:{}\n", location.file(), location.line());
    } else {
        log_emerg!("\tNo panic info\n");
    }

    unsafe { panic("Rust panic caused kernel to panic.\n".as_ptr() as *const c_char) }
}
