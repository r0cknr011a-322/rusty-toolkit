use core::arch::asm;

pub struct RiscV { }

impl RiscV {
    pub fn set_sp(value: u64) {
        unsafe {
            asm!("addi sp, {}, 0", in(reg) value);
        }
    }
}
