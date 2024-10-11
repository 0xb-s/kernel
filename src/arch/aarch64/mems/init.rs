use crate::arch::aarch64::mems::access_control::AccessControl;
use crate::arch::aarch64::mems::allocator::BuddyAllocator;

pub struct MemoryInitializer {
    allocator: BuddyAllocator,
    access_control: AccessControl,
}

impl MemoryInitializer {
    pub fn new(base_address: usize, total_size: usize) -> Self {
        let mut allocator = BuddyAllocator {
            free_lists: [core::ptr::null_mut(); 11],
            base_address,
            total_size,
        };
        allocator.init(base_address, total_size);

        let access_control = AccessControl::new();

        Self {
            allocator,
            access_control,
        }
    }

    pub fn setup(&mut self) {
        self.access_control.enable_user_mode();

        let kernel_region_base = self.allocator.base_address;
        self.access_control
            .set_access_for_region(0, kernel_region_base);

        let user_region_base = kernel_region_base + 0x100000;
        self.access_control
            .set_access_for_region(1, user_region_base);
    }

    pub fn get_allocator(&mut self) -> &mut BuddyAllocator {
        &mut self.allocator
    }

    pub fn get_access_control(&mut self) -> &mut AccessControl {
        &mut self.access_control
    }
}
