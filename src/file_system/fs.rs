use alloc::sync::Arc;
extern crate alloc;
use spin::Mutex;

use super::superblock::Superblock;

pub struct FileSystem {
    name: &'static str,
    superblock: Arc<Mutex<Superblock>>,
}

impl FileSystem {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            superblock: Arc::new(Mutex::new(Superblock::new(name))),
        }
    }

    pub fn mount(&self) -> Result<(), &'static str> {
        let mut superblock = self.superblock.lock();
        superblock.mount()?;
        Ok(())
    }

    pub fn unmount(&self) {
        let mut superblock = self.superblock.lock();
        superblock.unmount();
    }
}
