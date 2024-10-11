use bitflags::bitflags;
use core::arch::asm;
extern crate alloc;
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EntryFlags: u64 {
        const VALID = 0b00000001;
        const READ = 0b00000010;
        const WRITE = 0b00000100;
        const EXECUTE = 0b00001000;
        const USER = 0b00010000;
        const TABLE = 0b00100000;
    }
}

#[derive(Clone, Copy)]
pub enum SegmentInfo {
    Size = 0x1000,
    IndexBits = 9,
    Levels = 3,
    OffsetWidth = 12,
    AddrWidth = 39,
}

pub trait SegmentManager {
    fn set_segment(&self, id: usize, addr: usize);
    fn generate_entry(index: usize, flags: EntryFlags) -> usize;
    fn extract_index(entry: usize) -> usize;
    fn retrieve_flags(entry: usize) -> Option<EntryFlags>;
    fn is_flag_set(entry: usize, flag: EntryFlags) -> bool;
    fn enable_flag(entry: usize, flag: EntryFlags) -> usize;
    fn disable_flag(entry: usize, flag: EntryFlags) -> usize;
}

pub struct MemoryController {
    pub system_register: SystemRegister,
}

impl MemoryController {
    pub fn new() -> Self {
        Self {
            system_register: SystemRegister::new(),
        }
    }
}

impl SegmentManager for MemoryController {
    fn set_segment(&self, id: usize, addr: usize) {
        self.system_register.write(
            self.system_register.identifier_val(id as u64)
                + self
                    .system_register
                    .base_address_val((addr << SegmentInfo::OffsetWidth as usize) as u64),
        );
        unsafe {
            refresh_tlb(id);
        }
    }

    fn generate_entry(index: usize, flags: EntryFlags) -> usize {
        let mut entry: usize = 0;
        entry |= 0x1;
        entry |= (0x1 << 1);

        if !flags.contains(EntryFlags::TABLE) {
            entry |= (0x1 << 2);
            entry |= (0b11 << 8);
            entry |= (0b1 << 10);
        } else {
            entry |= (3usize << 53);
        }

        if flags.contains(EntryFlags::USER) {
            entry |= (0b1 << 6);
        }

        if !flags.contains(EntryFlags::WRITE) {
            entry |= (0b1 << 7);
        }

        let masked_index = index & 0x0000_0000_000F_FFFF;
        entry |= (masked_index << SegmentInfo::OffsetWidth as usize);

        if flags.contains(EntryFlags::EXECUTE) {
            if flags.contains(EntryFlags::USER) {
                entry |= (0usize << 53);
            } else {
                entry |= (2usize << 53);
            }
        } else {
            entry |= (3usize << 53);
        }

        entry
    }

    fn extract_index(entry: usize) -> usize {
        (entry >> SegmentInfo::OffsetWidth as usize) & 0x0000_0000_000F_FFFF
    }

    fn retrieve_flags(entry: usize) -> Option<EntryFlags> {
        let mut flags = EntryFlags::empty();
        if entry & 1 > 0 {
            flags.insert(EntryFlags::VALID);
        }

        let access = (entry & (3 << 6)) >> 6;
        match access {
            0 => flags.insert(EntryFlags::READ | EntryFlags::WRITE),
            1 => flags.insert(EntryFlags::READ | EntryFlags::WRITE | EntryFlags::USER),
            2 => flags.insert(EntryFlags::READ),
            3 => flags.insert(EntryFlags::READ | EntryFlags::USER),
            _ => (),
        }

        let exec = (entry & (3 << 53)) >> 53;
        match exec {
            0 => flags.insert(EntryFlags::EXECUTE | EntryFlags::USER),
            2 => flags.insert(EntryFlags::EXECUTE),
            _ => (),
        }

        Some(flags)
    }

    fn is_flag_set(entry: usize, flag: EntryFlags) -> bool {
        match flag {
            EntryFlags::VALID | EntryFlags::READ => entry & 1 > 0,
            EntryFlags::TABLE => entry & (1 << 1) > 0,
            EntryFlags::WRITE => (entry & (1 << 7)) == 0,
            EntryFlags::EXECUTE => {
                let exec = (entry & (3 << 53)) >> 53;
                exec != 3
            }
            _ => false,
        }
    }

    fn enable_flag(entry: usize, flag: EntryFlags) -> usize {
        let mut modified_entry = entry;
        let current_flags = Self::retrieve_flags(entry).unwrap();

        if flag.contains(EntryFlags::VALID) {
            modified_entry |= 0x1;
        }

        if flag.contains(EntryFlags::WRITE) {
            modified_entry &= !(0b1 << 7);
        }

        if flag.contains(EntryFlags::READ) {
            if current_flags.contains(EntryFlags::USER) {
                modified_entry &= !(0b11 << 6);
                modified_entry |= (0b01 << 6);
            }
        }

        if flag.contains(EntryFlags::EXECUTE) {
            modified_entry &= !(3 << 53);
            if current_flags.contains(EntryFlags::USER) {
                modified_entry |= 0 << 53;
            } else {
                modified_entry |= 2 << 53;
            }
        }

        modified_entry
    }

    fn disable_flag(entry: usize, flag: EntryFlags) -> usize {
        let mut modified_entry = entry;
        let current_flags = Self::retrieve_flags(entry).unwrap();

        if flag.contains(EntryFlags::VALID) || flag.contains(EntryFlags::READ) {
            modified_entry &= !1;
        }

        if flag.contains(EntryFlags::TABLE) {
            modified_entry &= !(1 << 1);
        }

        if flag.contains(EntryFlags::WRITE) {
            modified_entry &= !(3 << 6);
            if current_flags.contains(EntryFlags::USER) {
                modified_entry |= 1 << 6;
            } else {
                modified_entry |= 3 << 6;
            }
        }

        if flag.contains(EntryFlags::EXECUTE) {
            modified_entry &= !(3 << 53);
            if current_flags.contains(EntryFlags::USER) {
                modified_entry |= 2 << 53;
            } else {
                modified_entry |= 3 << 53;
            }
        }

        modified_entry
    }
}

extern "C" {
    pub fn refresh_tlb(identifier: usize);
}

#[repr(C)]
pub struct SystemRegister {
    identifier: u64,
    base_address: u64,
}

impl SystemRegister {
    pub fn new() -> Self {
        SystemRegister {
            identifier: 0,
            base_address: 0,
        }
    }

    pub fn write(&self, value: u64) {
        unsafe {
            asm!("msr ttbr0_el1, {}", in(reg) value);
        }
    }

    pub fn identifier_val(&self, id: u64) -> u64 {
        id << 0
    }

    pub fn base_address_val(&self, addr: u64) -> u64 {
        addr << 1
    }
}
