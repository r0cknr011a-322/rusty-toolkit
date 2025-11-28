use core::arch::asm;

pub struct ARMv7A { }

impl ARMv7A {
	pub fn get_status() -> u32 {
		let cpsr: u32;
		unsafe { asm!("mrs {}, CPSR", out(reg) cpsr); }
		cpsr
	}

/*
	pub fn get_stack(&self) -> u32 {
		let sp: u32;
		sp
	}

	pub fn get_mmfr(&self) -> u32 {
		let id: u32;
		unsafe { asm!("mrc p15, 0, {}, c0, c1, 4", out(reg) id); }
		id
	}

	pub fn get_vtor(&self) -> u32 {
		let id: u32;
		unsafe { asm!("mrc p15, 0, {}, c12, c0, 0", out(reg) id); }
		id
	}
*/
}
