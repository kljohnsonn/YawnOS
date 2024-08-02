use core::mem::size_of;

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct GDTEntry {
    limit: u16,
    base: u16,
    base2: u8,
    access: u8,
    limit2_flags: u8,
    base3: u8,
}

#[repr(C, packed)]
struct GlobalDescriptor {
    size: u16,
    offset: u64,
}

static mut GDTR: GlobalDescriptor = GlobalDescriptor { size: 0, offset: 0 };
static mut TABLE: [GDTEntry; 5] = [GDTEntry {
    limit: 0,
    base: 0,
    base2: 0,
    access: 0,
    limit2_flags: 0,
    base3: 0,
}; 5];

fn create_entry(base: u16, limit: u32, access: u8, flags: u8) -> GDTEntry {
    GDTEntry {
        limit: (limit & 0xFFFF) as u16,
        base,
        base2: 0,
        access,
        limit2_flags: (((limit >> 16) & 0xF) as u8) | (flags << 4),
        base3: 0,
    }
}

fn create_table() {
    let entries = [
        create_entry(0, 0, 0, 0),            // Null
        create_entry(0, 0xFFFFF, 0x9A, 0xA), // Kernel Mode Code Segment
        create_entry(0, 0xFFFFF, 0x92, 0xC), // Kernel Mode Data Segment
        create_entry(0, 0xFFFFF, 0xFA, 0xA), // User Mode Code Segment
        create_entry(0, 0xFFFFF, 0xF2, 0xC), // User Mode Data Segment
    ];

    unsafe {
        TABLE.copy_from_slice(&entries);
    }
}

fn create_descriptor() -> GlobalDescriptor {
    GlobalDescriptor {
        size: (size_of::<GDTEntry>() * 5 - 1) as u16,
        offset: unsafe { TABLE.as_ptr() as u64 },
    }
}

#[inline(never)]
pub fn load_gdt() {
    create_table();
    unsafe {
        GDTR = create_descriptor();
        core::arch::asm!(
            "lgdt [{}]",
            "push 0x08",
            "lea rax, [rip + 2f]",
            "push rax",
            "retfq",
            "2:",
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            in(reg) &GDTR,
            options(nomem, nostack)
        );
    }
}
