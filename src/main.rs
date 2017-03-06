#![feature(lang_items)]
#![feature(start)]
#![no_std]

use core::ptr;
use core::slice;

const WIDTH: usize = 240;
const HEIGHT: usize = 160;

const VRAM_ADDR: usize = 0x06_000_000;

#[lang="panic_fmt"]
extern fn panic_fmt(_args: &core::fmt::Arguments,
                    _file: &str,
                    _line: u32) -> ! {
    loop {}
}

#[lang="eh_personality"]
pub extern fn eh_personality() -> ! { loop {} }

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {
    loop {}
}

fn vram() -> &'static mut [u16] {
    unsafe {
        slice::from_raw_parts_mut(VRAM_ADDR as *mut u16, WIDTH * HEIGHT)
    }
}

#[start]
pub fn entry(_: isize, _: *const *const u8) -> isize {
    let p = 0x04000000 as *mut u32;
    let mut vram = vram();
    unsafe {
        ptr::write_volatile(p, 0x0403);
    }

    for y in 0..HEIGHT {
        for x in 0..20 {
            vram[y * WIDTH + x] = 0x03E0;
        }
    }
    loop {}
}
