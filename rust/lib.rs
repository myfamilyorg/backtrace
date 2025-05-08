#![no_std]

extern crate ffi;
use core::fmt::Result as FmtResult;
use core::fmt::{Debug, Formatter};
use core::ptr::{null, null_mut};
use core::slice::from_raw_parts;
use core::str::from_utf8_unchecked;
use ffi::{backtrace, gen_backtrace, getenv};

pub const MAX_BACKTRACE_ENTRIES: usize = 128;

#[repr(C)]
#[derive(Clone)]
pub struct Backtrace {
    entries: [*mut (); MAX_BACKTRACE_ENTRIES],
    size: i32,
}

impl Debug for Backtrace {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        unsafe {
            let bt = self.as_ptr();
            let s = if bt.is_null() {
                "Backtrace disabled. To enable export RUST_BACKTRACE=1."
            } else {
                let len = ffi::cstring_len(bt);
                let slice = from_raw_parts(bt, len as usize);
                from_utf8_unchecked(slice)
            };
            match write!(f, "{}", s) {
                Ok(_) => {
                    ffi::release(bt);
                    Ok(())
                }

                Err(e) => {
                    ffi::release(bt);
                    Err(e)
                }
            }
        }
    }
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
