#![no_std]

use core::ptr::{ self };
use core::mem::{ self };
use core::slice::{ self };
use core::sync::atomic::{ AtomicU8, AtomicU16, AtomicU32, Ordering };

pub trait RW8 {
    fn rd8(&mut self, off: usize) -> u8;
    fn wr8(&mut self, off: usize, value: u8);
}

pub trait RWVolatile8 {
    fn rd8_volatile(&mut self, off: usize) -> u8;
    fn wr8_volatile(&mut self, off: usize, value: u8);
}

pub trait RWAtomic8 {
    fn rd8_atomic(&mut self, off: usize) -> u8;
    fn wr8_atomic(&mut self, off: usize, value: u8); 
}

pub trait RW16 {
    fn rd16(&mut self, off: usize) -> u16;
    fn wr16(&mut self, off: usize, value: u16);
}

pub trait RWVolatile16 {
    fn rd16_volatile(&mut self, off: usize) -> u16;
    fn wr16_volatile(&mut self, off: usize, value: u16);
}

pub trait RWAtomic16 {
    fn rd16_atomic(&mut self, off: usize) -> u16;
    fn wr16_atomic(&mut self, off: usize, value: u16);
}

pub trait RW32 {
    fn rd32(&mut self, off: usize) -> u32;
    fn wr32(&mut self, off: usize, value: u32);
}

pub trait RWVolatile32 {
    fn rd(&mut self, off: usize) -> u64;
    fn wr(&mut self, off: usize, value: u64);
}

pub trait RWAtomic32 {
    fn rd(&mut self, off: usize) -> u64;
    fn wr(&mut self, off: usize, value: u64);
}

pub trait RWBuf {
    fn from(&mut self, off: usize, data: &[u8]);
    fn to(&mut self, off: usize, data: &mut [u8]);
}

pub struct IOBufMem<'a> {
    buf: &'a mut [u8],
}

impl IOBufMem<'_> {
    pub unsafe fn new(base: usize, len: usize) -> Self {
        Self {
            buf: slice::from_raw_parts_mut(ptr::without_provenance_mut(base), len),
        }
    }

    fn off<T>(&self, off: usize) -> *const T {
        (&self.buf[off..off + mem::size_of::<T>()]).as_ptr().cast::<T>()
    }

    fn off_mut<T>(&self, off: usize) -> *mut T {
        (&self.buf[off..]).as_ptr().cast::<T>().cast_mut()
    }
}

impl RW8 for IOBufMem<'_> {
    fn rd8(&mut self, off: usize) -> u8 {
        unsafe {
            let addr = self.off::<u8>(off);
            addr.read()
        }
    }

    fn wr8(&mut self, off: usize, value: u8) {
        unsafe {
            let addr = self.off_mut::<u8>(off);
            addr.write(value);
        }
    }
}

impl RWVolatile8 for IOBufMem<'_> {
    fn rd8_volatile(&mut self, off: usize) -> u8 {
        unsafe {
            let addr = self.off::<u8>(off);
            addr.read_volatile()
        }
    }

    fn wr8_volatile(&mut self, off: usize, value: u8) {
        unsafe {
            let addr = self.off_mut::<u8>(off);
            addr.write_volatile(value);
        }
    }
}

impl RWVolatile8 for &mut IOBufMem<'_> {
    fn rd8_volatile(&mut self, off: usize) -> u8 {
        unsafe {
            let addr = self.off::<u8>(off);
            addr.read_volatile()
        }
    }

    fn wr8_volatile(&mut self, off: usize, value: u8) {
        unsafe {
            let addr = self.off_mut::<u8>(off);
            addr.write_volatile(value);
        }
    }
}

#[cfg(target_has_atomic = "8")]
impl RWAtomic8 for IOBufMem<'_> {
    fn rd8_atomic(&mut self, off: usize) -> u8 {
        unsafe {
            let data = AtomicU8::from_ptr(self.off_mut::<u8>(off));
            data.load(Ordering::SeqCst)
        }
    }

    fn wr8_atomic(&mut self, off: usize, value: u8) {
        unsafe {
            let addr = self.off_mut::<u8>(off);
            addr.write_volatile(value);
        }
    }
}

/*
impl RW16 for IOBufVolatile<'_> {
    fn rd(&mut self, off: usize) -> u16 {
        unsafe {
            let addr = self.off::<u16>(off);
            addr.read_volatile()
        }
    }

    fn wr(&mut self, off: usize, value: u16) {
        unsafe {
            let addr = self.off_mut::<u16>(off);
            addr.write_volatile(value);
        }
    }
}

impl RW32 for IOBufVolatile<'_> {
    fn rd(&mut self, off: usize) -> u32 {
        unsafe {
            let addr = self.off::<u32>(off);
            addr.read_volatile()
        }
    }

    fn wr(&mut self, off: usize, value: u32) {
        unsafe {
            let addr = self.off_mut::<u32>(off);
            addr.write_volatile(value);
        }
    }
}
*/
