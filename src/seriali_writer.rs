use core::fmt::{self, Write};
use x86_64::instructions::port::Port;

const SERIAL_PORT_COM1: u16 = 0x3F8;

pub struct SerialWriter {
    port: Port<u8>,
}
impl SerialWriter {
    pub fn new() -> Self {
        let mut writer = SerialWriter {
            port: Port::new(SERIAL_PORT_COM1),
        };
        writer.init();
        writer
    }

    fn init(&mut self) {
        unsafe { self.port.write(0x00) };

        unsafe {
            Port::new(SERIAL_PORT_COM1 + 1).write(0x00);
            Port::new(SERIAL_PORT_COM1 + 3).write(0x80);
            Port::new(SERIAL_PORT_COM1).write(0x03);
            Port::new(SERIAL_PORT_COM1 + 1).write(0x00);
        }

        unsafe { Port::new(SERIAL_PORT_COM1 + 3).write(0x03) };

        unsafe { Port::new(SERIAL_PORT_COM1 + 2).write(0xC7) };

        unsafe { Port::new(SERIAL_PORT_COM1 + 4).write(0x0B) };
    }

    pub fn write_byte(&mut self, byte: u8) {
        while !self.is_transmit_empty() {}
        unsafe { self.port.write(byte) };
    }

    fn is_transmit_empty(&self) -> bool {
        let status: u8 = unsafe { Port::new(SERIAL_PORT_COM1 + 5).read() };
        status & 0x20 != 0
    }
}
impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}
pub static mut SERIAL_WRITER: SerialWriter = SerialWriter {
    port: Port::new(SERIAL_PORT_COM1),
};
pub fn serial_print(args: fmt::Arguments) {
    unsafe {
        SERIAL_WRITER.write_fmt(args).unwrap();
    }
}
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial_writer::serial_print(format_args!($($arg)*));
    };
}
