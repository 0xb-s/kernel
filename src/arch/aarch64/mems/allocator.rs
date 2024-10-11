use core::ptr::null_mut;

const MAX_ORDER: usize = 10;

pub(in crate::arch::aarch64::mems) struct Block {
    next: *mut Block,
}

pub struct BuddyAllocator {
    pub(in crate::arch::aarch64::mems) free_lists: [*mut Block; MAX_ORDER + 1],
    pub(in crate::arch::aarch64::mems) base_address: usize,
    pub(in crate::arch::aarch64::mems) total_size: usize,
}

impl BuddyAllocator {
    pub fn init(&mut self, base: usize, size: usize) {
        self.base_address = base;
        self.total_size = size;

        for i in 0..=MAX_ORDER {
            self.free_lists[i] = null_mut();
        }

        self.add_block_to_list(base, MAX_ORDER);
    }

    pub fn allocate(&mut self, size: usize) -> Option<*mut u8> {
        let order = self.find_order(size)?;
        self.split_block(order)
    }

    pub fn deallocate(&mut self, addr: *mut u8, size: usize) {
        let order = self
            .find_order(size)
            .expect("Invalid block size for deallocation.");
        self.add_block_to_list(addr as usize, order);
        self.merge_blocks(order);
    }

    fn add_block_to_list(&mut self, addr: usize, order: usize) {
        let block = addr as *mut Block;
        unsafe {
            (*block).next = self.free_lists[order];
        }
        self.free_lists[order] = block;
    }

    fn split_block(&mut self, order: usize) -> Option<*mut u8> {
        for current_order in order..=MAX_ORDER {
            if !self.free_lists[current_order].is_null() {
                let block = self.free_lists[current_order];
                self.free_lists[current_order] = unsafe { (*block).next };

                for i in (order..current_order).rev() {
                    let buddy = (block as usize) + (1 << i) * self.block_size(0);
                    self.add_block_to_list(buddy, i);
                }

                return Some(block as *mut u8);
            }
        }
        None
    }

    fn merge_blocks(&mut self, order: usize) {
        let mut current = self.free_lists[order];

        while !current.is_null() {
            let buddy_addr = self.find_buddy(current as usize, order);
            let mut buddy_found = false;

            let mut prev = null_mut();
            let mut node = self.free_lists[order];
            while !node.is_null() {
                if node as usize == buddy_addr {
                    buddy_found = true;
                    break;
                }
                prev = node;
                node = unsafe { (*node).next };
            }

            if buddy_found {
                if !prev.is_null() {
                    unsafe {
                        (*prev).next = (*node).next;
                    }
                } else {
                    self.free_lists[order] = unsafe { (*node).next };
                }

                let merged_addr = usize::min(current as usize, buddy_addr);
                self.add_block_to_list(merged_addr, order + 1);
                current = self.free_lists[order];
            } else {
                break;
            }
        }
    }

    fn find_buddy(&self, addr: usize, order: usize) -> usize {
        addr ^ (1 << (order + self.block_size(0)))
    }

    fn find_order(&self, size: usize) -> Option<usize> {
        let mut order = 0;
        let mut block_size = self.block_size(order);
        while block_size < size {
            order += 1;
            block_size = self.block_size(order);
            if order > MAX_ORDER {
                return None;
            }
        }
        Some(order)
    }

    fn block_size(&self, order: usize) -> usize {
        1 << (order + self.block_size(0))
    }
}
