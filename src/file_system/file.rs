use super::inode::Inode;

pub struct File {
    inode: Inode,
    cursor: usize,
}

impl File {
    pub fn new(inode: Inode) -> Self {
        Self { inode, cursor: 0 }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        let bytes_read = self.inode.read(self.cursor as u64, buffer)?;
        self.cursor += bytes_read;
        Ok(bytes_read)
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<usize, &'static str> {
        let bytes_written = self.inode.write(self.cursor as u64, buffer)?;
        self.cursor += bytes_written;
        Ok(bytes_written)
    }
}
