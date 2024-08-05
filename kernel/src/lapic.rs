use core::cell::UnsafeCell;
use core::isize;
use limine::request::{self, HhdmRequest};
use limine::response::{self, HhdmResponse};

const ID: usize = 0x0020 / 4; // ID
const VER: usize = 0x0030 / 4; // Version
const TPR: usize = 0x0080 / 4; // Task Priority
const EOI: usize = 0x00B0 / 4; // EOI
const SVR: usize = 0x00F0 / 4; // Spurious Interrupt Vector
const ENABLE: u32 = 0x00000100; // Unit Enable
const ESR: usize = 0x0280 / 4; // Error Status
const ICRLO: usize = 0x0300 / 4; // Interrupt Command
const INIT: u32 = 0x00000500; // INIT/RESET
const STARTUP: u32 = 0x00000600; // Startup IPI
const DELIVS: u32 = 0x00001000; // Delivery status
const ASSERT: u32 = 0x00004000; // Assert interrupt (vs deassert)
const DEASSERT: u32 = 0x00000000;
const LEVEL: u32 = 0x00008000; // Level triggered
const BCAST: u32 = 0x00080000; // Send to all APICs, including self.
const BUSY: u32 = 0x00001000;
const FIXED: u32 = 0x00000000;
const ICRHI: usize = 0x0310 / 4; // Interrupt Command [63:32]
const TIMER: usize = 0x0320 / 4; // Local Vector Table 0 (TIMER)
const X1: u32 = 0x0000000B; // divide counts by 1
const PERIODIC: u32 = 0x00020000; // Periodic
const PCINT: usize = 0x0340 / 4; // Performance Counter LVT
const LINT0: usize = 0x0350 / 4; // Local Vector Table 1 (LINT0)
const LINT1: usize = 0x0360 / 4; // Local Vector Table 2 (LINT1)
const ERROR: usize = 0x0370 / 4; // Local Vector Table 3 (ERROR)
const MASKED: u32 = 0x00010000; // Interrupt masked
const TICR: usize = 0x0380 / 4; // Timer Initial Count
const TCCR: usize = 0x0390 / 4; // Timer Current Count
const TDCR: usize = 0x03E0 / 4; // Timer Divide Configuration
const IRQ_TIMER: u32 = 0;
const IRQ_KBD: u32 = 1;
const IRQ_COM1: u32 = 4;
const IRQ_IDE: u32 = 14;
const IRQ_ERROR: u32 = 19;
const IRQ_SPURIOUS: u32 = 31;
const T_IRQ0: u32 = 32; // IRQ 0 corresponds to int T_IRQ
const APIC_APICID: usize = 0x20; // APIC ID Register
const APIC_APICVER: usize = 0x30; // APIC Version Register
const APIC_TASKPRIOR: usize = 0x80; // Task Priorities Register
const APIC_EOI: usize = 0xB0; // End of Interrupt Register
const APIC_LDR: usize = 0xD0; // Local Descriptor Register
const APIC_DFR: usize = 0xE0; // Destination Format Register
const APIC_SPURIOUS: usize = 0xF0; // Spurious Interrupt Vector Register
const APIC_ESR: usize = 0x280; // Error Status Register
const APIC_ICRL: usize = 0x300; // Interrupt Command Register Low
const APIC_ICRH: usize = 0x310; // Interrupt Command Register High
const APIC_LVT_TMR: usize = 0x320; // Local Vector Table - Timer
const APIC_LVT_PERF: usize = 0x340; // Local Vector Table - Performance Monitoring
const APIC_LVT_LINT0: usize = 0x350; // Local Vector Table - LINT0
const APIC_LVT_LINT1: usize = 0x360; // Local Vector Table - LINT1
const APIC_LVT_ERR: usize = 0x370; // Local Vector Table - Error
const APIC_TMRINITCNT: usize = 0x380; // Timer Initial Count Register
const APIC_TMRCURRCNT: usize = 0x390; // Timer Current Count Register
const APIC_TMRDIV: usize = 0x3E0; // Timer Divide Configuration Register
const APIC_LAST: usize = 0x38F; // Last Register Address
const APIC_DISABLE: usize = 0x10000; // Disable Register
const APIC_SW_ENABLE: usize = 0x100; // Software Enable Register
const APIC_CPUFOCUS: usize = 0x200; // CPU Focus Register
const APIC_NMI: usize = 4 << 8; // Non-Maskable Interrupt Register
const TMR_PERIODIC: u32 = 0x20000; // Timer Periodic Register
const TMR_BASEDIV: usize = 1 << 20; // Timer Base Divider Register

static HHDMREQUEST: HhdmRequest = HhdmRequest::new();

pub unsafe fn init_lapic() {
    lapicw(0x0080, 0);

    lapicw(0x00d0, 0x01000000);
    lapicw(0x00e0, 0xffffffff);

    lapicw(0x00f0, 0x100 | 0xff)
}

pub unsafe fn get_hhdm_offset() -> Option<u64> {
    let response = HHDMREQUEST.get_response();

    if let Some(resp) = response {
        return Some(resp.offset());
    } else {
        return None;
    }
}

pub unsafe fn lapicw(index: usize, value: u32) {
    let lapic = (get_hhdm_offset().unwrap() + 0xFEE0000) as *mut u32;
    let offset = lapic.add(index);
    offset.write(value);
}

pub unsafe fn lapicr(index: usize) -> u32 {
    let lapic = (get_hhdm_offset().unwrap() + 0xFEE0000) as *mut u32;
    let offset = lapic.add(index);
    offset.read()
}
