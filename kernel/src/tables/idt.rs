use crate::interrupts::interrupts::*;
use core::arch::asm;
use core::cell::UnsafeCell;
use core::u64;

#[repr(C, align(0x10))]
#[derive(Copy, Clone)]
struct IDTEntry {
    isr_low: u16,
    kernel_cs: u16,
    ist: u8,
    attributes: u8,
    isr_mid: u16,
    isr_high: u32,
    reserved: u32,
}

unsafe impl Sync for IDTEntry {}
unsafe impl Sync for IDTTable {}
unsafe impl Sync for IDTR {}

#[repr(transparent)]
struct IDTTable(UnsafeCell<[IDTEntry; 256]>);

static IDT_TABLE: IDTTable = IDTTable(UnsafeCell::new([IDTEntry::zeroed(); 256]));

#[repr(C, packed)]
struct IDTR {
    limit: u16,
    base: *mut [IDTEntry; 256],
}
static IDTR: IDTR = IDTR {
    limit: ((size_of::<IDTEntry>() as u16) * 256 - 1),
    base: IDT_TABLE.0.get(),
};

impl IDTTable {
    pub unsafe fn set_entry(
        &'static self,
        index: u8,
        flags: u8,
        f: unsafe extern "x86-interrupt" fn(),
    ) {
        self.0
            .get()
            .cast::<IDTEntry>()
            .offset(index as isize)
            .write(IDTEntry::new(f, flags))
    }
}

impl IDTEntry {
    pub const fn zeroed() -> Self {
        IDTEntry {
            isr_low: 0,
            kernel_cs: 0,
            ist: 0,
            attributes: 0,
            isr_mid: 0,
            isr_high: 0,
            reserved: 0,
        }
    }

    pub unsafe fn new(f: unsafe extern "x86-interrupt" fn(), flags: u8) -> Self {
        let pointer = f as u64;
        IDTEntry {
            isr_low: (pointer as u64) as u16,
            kernel_cs: 0x0008,
            ist: 0,
            attributes: flags,
            isr_mid: ((pointer as u64) >> 16) as u16,
            isr_high: ((pointer as u64) >> 32) as u32,
            reserved: 0,
        }
    }
}

#[inline(never)]
pub unsafe fn load_idt() {
    IDTTable::set_entry(&IDT_TABLE, 0, 0x8F, divide_by_zero_err);

    asm!(
        "lidt [{}]",
        in(reg) &IDTR,
        options(readonly, nostack)
    );

    asm!("sti")
}
