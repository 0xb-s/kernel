use super::controller::SpiController;
extern crate alloc;
use alloc::vec::Vec;

pub struct SpiManager {
    controllers: Vec<SpiController>,
}

impl SpiManager {
    pub fn new() -> Self {
        Self {
            controllers: Vec::new(),
        }
    }

    pub fn add_controller(&mut self, controller: SpiController) {
        self.controllers.push(controller);
    }

    pub fn get_controller(&self, bus_num: u16) -> Option<&SpiController> {
        self.controllers.iter().find(|c| c.bus_num == bus_num)
    }

    pub fn get_controller_mut(&mut self, bus_num: u16) -> Option<&mut SpiController> {
        self.controllers.iter_mut().find(|c| c.bus_num == bus_num)
    }
}
