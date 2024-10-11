use alloc::vec::Vec;
extern crate alloc;

#[derive(Default)]
pub struct SpiStatistics {
    pub transfers: usize,
    pub errors: usize,
    pub bytes_tx: u64,
    pub bytes_rx: u64,
}

pub struct SpiTransfer<'a> {
    pub tx_buf: Option<&'a [u8]>,
    pub rx_buf: Option<&'a mut [u8]>,
    pub len: usize,
    pub speed_hz: u32,
}

impl<'a> SpiTransfer<'a> {
    pub fn new(
        tx_buf: Option<&'a [u8]>,
        rx_buf: Option<&'a mut [u8]>,
        len: usize,
        speed_hz: u32,
    ) -> Self {
        Self {
            tx_buf,
            rx_buf,
            len,
            speed_hz,
        }
    }
}

pub struct SpiMessage<'a> {
    pub transfers: Vec<SpiTransfer<'a>>,
}

impl<'a> SpiMessage<'a> {
    pub fn new() -> Self {
        Self {
            transfers: Vec::new(),
        }
    }

    pub fn add_transfer(&mut self, transfer: SpiTransfer<'a>) {
        self.transfers.push(transfer);
    }
}
