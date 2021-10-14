#![no_std]

#[panic_handler]
fn hnd(_: &core::panic::PanicInfo) -> ! { unsafe { core::arch::wasm32::unreachable() } }

#[no_mangle]
pub extern fn a() { b(); }
#[inline(never)]
fn b() { c(); }
#[inline(never)]
fn c() { panic!(); }

