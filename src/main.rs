
#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
panic!("Some panic message");
    loop {}
}

