#![no_std]

#[allow(non_camel_case_types, non_upper_case_globals, unused, non_snake_case)]
pub mod ffi;

#[no_mangle]
pub extern "C" fn rust_main() -> u32 {
    let msg = b"[rust-rt_kprintf]: hello world\n\0";
    unsafe {
        crate::ffi::rt_kprintf(msg.as_ptr() as *const _);
    }
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}