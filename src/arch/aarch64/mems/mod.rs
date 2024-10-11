use alloc::string::{String, ToString};
use core::marker::Copy;
use core::{arch::asm, ffi::CStr, mem};
pub mod access_control;
pub mod allocator;
pub mod controller;
pub mod init;
extern crate alloc;
pub enum DataType {
    U8(u8),
    Usize(usize),
    CString(*const i8),
    RawBlock(*const u8, usize),
}

#[repr(C)]
pub union DataUnion {
    u8_val: u8,
    usize_val: usize,
    cstr_ptr: *const i8,
    raw_ptr: *const u8,
}

pub trait Copier {
    fn copy(&self) -> Result<(), &'static str>;
}

pub struct AccessControl {
    pub enabled: bool,
}

impl AccessControl {
    pub fn new() -> Self {
        Self { enabled: false }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

pub struct MemoryOperation {
    pub source: DataType,
    pub destination: *mut u8,
    pub access_control: AccessControl,
}

impl MemoryOperation {
    pub fn new(source: DataType, destination: *mut u8) -> Self {
        Self {
            source,
            destination,
            access_control: AccessControl::new(),
        }
    }
}

impl Copier for MemoryOperation {
    fn copy(&self) -> Result<(), &'static str> {
        if !self.access_control.is_enabled() {
            return Err("User access is disabled");
        }

        match &self.source {
            DataType::U8(value) => {
                if self.destination.is_null() {
                    return Err("Destination pointer is null");
                }
                unsafe {
                    *self.destination = *value;
                }
            }
            DataType::Usize(value) => {
                let dst = self.destination as *mut usize;
                if dst.is_null() {
                    return Err("Destination pointer is null");
                }
                unsafe {
                    *dst = *value;
                }
            }
            DataType::CString(ptr) => {
                if ptr.is_null() {
                    return Err("Source pointer is null");
                }
            }
            DataType::RawBlock(ptr, size) => {
                if ptr.is_null() || self.destination.is_null() {
                    return Err("Null pointer encountered");
                }
                unsafe {
                    copy_raw_block(self.destination, *ptr, *size)?;
                }
            }
        }

        Ok(())
    }
}


pub unsafe fn copy_raw_block(to: *mut u8, from: *const u8, n: usize) -> Result<(), &'static str> {
    if to.is_null() || from.is_null() {
        return Err("Null pointer encountered");
    }

    let mut remaining = n;
    let mut src = from;
    let mut dst = to;

    while remaining > 0 {
        asm!(
            "ldrb w3, [{src}], #1",
            "strb w3, [{dst}], #1",
            "subs {remaining}, {remaining}, #1",
            src = inout(reg) src,
            dst = inout(reg) dst,
            remaining = inout(reg) remaining,
            options(nostack, preserves_flags),
        );
    }

    Ok(())
}
