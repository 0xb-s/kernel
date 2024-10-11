
use core::arch::asm;

pub struct AccessControl {
    user_mode_enabled: bool,
}

impl AccessControl {
    pub fn new() -> Self {
        Self {
            user_mode_enabled: false,
        }
    }

    pub fn enable_user_mode(&mut self) {
        self.user_mode_enabled = true;

        unsafe {
            asm!("msr sctlr_el1, {val}", val = in(reg) 0b1);
        }
    }

    pub fn disable_user_mode(&mut self) {
        self.user_mode_enabled = false;
        unsafe {
            asm!("msr sctlr_el1, {val}", val = in(reg) 0b0);
        }
    }

    pub fn is_user_mode_enabled(&self) -> bool {
        self.user_mode_enabled
    }

    pub unsafe fn flush_tlb(&self, asid: usize) {
        asm!("dsb ishst");
        asm!("tlbi aside1, {}", in(reg) asid);
        asm!("dsb ish");
        asm!("isb");
    }

    pub fn set_access_for_region(&self, id: usize, base_address: usize) {
        unsafe {
            asm!("msr ttbr0_el1, {addr}", addr = in(reg) (base_address | (id << 48)));
            self.flush_tlb(id);
        }
    }
}
