use alloc::vec::Vec;

use super::device::SpiDevice;
extern crate alloc;

pub struct SpiController {
    pub bus_num: u16,
    pub max_speed_hz: u32,
    pub num_chipselect: u8,
    pub devices: Vec<SpiDevice>,
}

impl SpiController {
    pub fn new(bus_num: u16, max_speed_hz: u32, num_chipselect: u8) -> Self {
        Self {
            bus_num,
            max_speed_hz,
            num_chipselect,
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: SpiDevice) {
        self.devices.push(device);
    }
}
