use core::arch::asm;

pub unsafe extern "x86-interrupt" fn divide_by_zero_err() {
    asm!("cli; hlt")
}
