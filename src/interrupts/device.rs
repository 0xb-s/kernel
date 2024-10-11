use super::transfer::SpiStatistics;

pub struct SpiDevice {
    pub chip_select: u8,
    pub max_speed_hz: u32,
    pub mode: u16,
    pub bits_per_word: u8,
    pub statistics: SpiStatistics,
}

impl SpiDevice {
    pub fn new(chip_select: u8, max_speed_hz: u32, mode: u16, bits_per_word: u8) -> Self {
        Self {
            chip_select,
            max_speed_hz,
            mode,
            bits_per_word,
            statistics: SpiStatistics::default(),
        }
    }

    pub fn configure(&mut self, max_speed_hz: u32, mode: u16, bits_per_word: u8) {
        self.max_speed_hz = max_speed_hz;
        self.mode = mode;
        self.bits_per_word = bits_per_word;
    }
}
