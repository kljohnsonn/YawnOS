#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
use core::arch::asm;
use core::char::from_u32;
mod cpuid;
mod interrupts;
mod lapic;
mod serial;
mod tables;
use cpuid::check_apic;
use lapic::init_lapic;
use limine::request::FramebufferRequest;
use limine::BaseRevision;
use serial::{Serial, PORT};
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
    let port = PORT::new(0x3F8);
    let serial = Serial::new(port).unwrap();
    serial.write("We are so back!\n");
    check_apic();
    init_lapic();
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
