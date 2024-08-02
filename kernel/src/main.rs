#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
use core::arch::asm;

mod interrupts;
mod tables;

use limine::request::FramebufferRequest;
use limine::BaseRevision;
use tables::gdt::load_gdt;
use tables::idt::load_idt;

/// Sets the base revision to the latest revision supported by the crate.
/// See specification for further info.
// Be sure to mark all limine requests with #[used], otherwise they may be removed by the compiler.
#[used]
// The .requests section allows limine to find the requests faster and more safely.
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.
    assert!(BASE_REVISION.is_supported());
    load_gdt();
    load_idt();
    asm!("int $0x00");
    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
