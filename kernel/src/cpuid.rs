use crate::serial::*;
use core::arch::asm;

pub unsafe fn cpuid_edx() -> u32 {
    let ret: u32;
    asm!(
    "mov edx, 01h",
    "cpuid",
    out("edx") ret,
    );

    ret
}

pub unsafe fn outbb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val);
}

pub unsafe fn disable_pci() {
    outbb(0x20 + 1, 0xff);
    outbb(0xA0 + 1, 0xff);
}

pub unsafe fn check_apic() {
    let info = cpuid_edx();
    let apic_bit: u32 = 9;
    let apic_available = (info & (1 << apic_bit)) != 0;

    if apic_available {
        Serial::new(PORT::new(0x3f8))
            .unwrap()
            .write("APIC is HERE!");
        disable_pci();
    } else {
    }
}

const IA32_APIC_BASE_MSR: u64 = 0x1b;

pub unsafe fn read_msr() {
    asm!(
    "mov ecx",
    "rdmsr",

    in ("ecx") IA32_APIC_BASE_MSR
    )
}
