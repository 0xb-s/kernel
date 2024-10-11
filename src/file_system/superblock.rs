extern crate alloc;
use alloc::boxed::Box;

use super::inode::Inode;

pub struct Superblock {
    name: &'static str,
    is_mounted: bool,
    root_inode: Option<Box<Inode>>,
}

impl Superblock {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            is_mounted: false,
            root_inode: None,
        }
    }

    pub fn mount(&mut self) -> Result<(), &'static str> {
        if self.is_mounted {
            return Err("Superblock already mounted");
        }

        self.root_inode = Some(Box::new(Inode::default()));
        self.is_mounted = true;
        Ok(())
    }

    pub fn unmount(&mut self) {
        if !self.is_mounted {
            return;
        }

        self.root_inode.take();
        self.is_mounted = false;
    }
}
