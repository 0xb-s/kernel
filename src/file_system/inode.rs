use alloc::vec::Vec;
use core::result::Result;
extern crate alloc;
#[derive(Debug, Default)]
pub struct Inode {
    pub size: u64,
    pub blocks: Vec<u8>,
}

impl Inode {
    pub fn new(size: u64, blocks: Vec<u8>) -> Self {
        Inode { size, blocks }
    }

    pub fn read(&self, offset: u64, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if offset >= self.size {
            return Err("Offset is beyond the size of the inode");
        }

        let available_bytes = (self.size - offset) as usize;
        let read_size = core::cmp::min(available_bytes, buffer.len());

        let start = offset as usize;
        let end = start + read_size;

        if end > self.blocks.len() {
            return Err("Read operation exceeds available blocks");
        }

        buffer[..read_size].copy_from_slice(&self.blocks[start..end]);

        Ok(read_size)
    }

    pub fn write(&mut self, offset: u64, buffer: &[u8]) -> Result<usize, &'static str> {
        if offset > self.size {
            return Err("Offset is beyond the size of the inode");
        }

        let write_size = buffer.len();
        let start = offset as usize;
        let end = start + write_size;

        if end > self.blocks.len() {
            self.blocks.resize(end, 0);
            self.size = end as u64;
        }

        self.blocks[start..end].copy_from_slice(buffer);

        Ok(write_size)
    }
}
