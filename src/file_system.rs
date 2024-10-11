use core::sync::atomic::{AtomicBool, Ordering};
extern crate alloc;
use alloc::sync::Arc;
mod file;
mod fs;
mod inode;
mod superblock;
use fs::FileSystem;
static IS_REGISTERED: AtomicBool = AtomicBool::new(false);

fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    loop {}
}

pub fn init_file_system() -> Result<(), &'static str> {
    let my_fs = Arc::new(FileSystem::new("myfs"));

    IS_REGISTERED.store(true, Ordering::Relaxed);
    Ok(())
}

pub fn shutdown_file_system() {
    if IS_REGISTERED.load(Ordering::Relaxed) {
        //TODO clear here
        IS_REGISTERED.store(false, Ordering::Relaxed);
    }
}
