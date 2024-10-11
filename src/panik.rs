use crate::serial_writer::serial_print;
use core::fmt::Write;
use core::panic::PanicInfo;

#[macro_export]
macro_rules! panikk {
    ($($arg:tt)*) => {
        $crate::panik::panic_with_message(format_args!($($arg)*));
    };
}
fn panic_with_message(args: core::fmt::Arguments) -> ! {
    serial_print!("\n*** KERNEL PANIC ***\n");

    serial_print!("Message: ");
    serial_print!("{}", args);
    serial_print!("\n");

    loop {
        x86_64::instructions::hlt();
    }
}

fn panic(info: &PanicInfo) -> ! {
    serial_print!(" KERNEL PANIC ");

    if let Some(location) = info.location() {
        serial_print!(
            "Panic occurred in file '{}' at line {}:\n",
            location.file(),
            location.line()
        );
    } else {
        serial_print!("Panic location unknown.\n");
    }

    if let Some(message) = info.message() {
        serial_print!("Message: {}\n", message);
    } else {
        serial_print!("No additional information provided.\n");
    }

    loop {
        x86_64::instructions::hlt();
    }
}
