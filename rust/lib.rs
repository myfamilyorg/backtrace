#![no_std]

pub extern crate ffi;

use core::ptr::{null, null_mut};
use ffi::{backtrace, gen_backtrace, getenv};
pub const MAX_BACKTRACE_ENTRIES: usize = 128;

#[repr(C)]
#[derive(Clone)]
pub struct Backtrace {
    entries: [*mut (); MAX_BACKTRACE_ENTRIES],
    size: i32,
}

impl Backtrace {
    pub fn new() -> Self {
        let mut ret = Self::init();
        ret.capture();
        ret
    }

    pub const fn init() -> Self {
        Self {
            entries: [null_mut(); MAX_BACKTRACE_ENTRIES],
            size: 0,
        }
    }

    pub fn capture(&mut self) {
        let size = unsafe {
            if getenv("RUST_BACKTRACE\0".as_ptr()).is_null() {
                0
            } else {
                backtrace(self.entries.as_mut_ptr(), MAX_BACKTRACE_ENTRIES as i32)
            }
        };
        self.size = size;
    }

    pub unsafe fn as_ptr(&self) -> *const u8 {
        if self.size <= 0 {
            null()
        } else {
            unsafe { gen_backtrace(self.entries.as_ptr(), self.size) }
        }
    }
}
