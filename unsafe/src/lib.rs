#![no_std]

use core::ptr::{ self };
use core::mem::{ self };
use core::slice::{ self };
use core::sync::atomic::{ AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering };

#[derive(Clone, Copy)]
pub struct RawBuf {
    buf: &'static [u8],
}

impl RawBuf {
    pub fn new(base: usize, len: usize) -> Self {
        Self {
            buf: unsafe {
                slice::from_raw_parts(ptr::without_provenance(base), len)
            },
        }
    }

    fn off<T>(&self, off: usize) -> *const T {
        (&self.buf[off..off+mem::size_of::<T>()]).as_ptr().cast::<T>()
    }

    fn off_mut<T>(&self, off: usize) -> *mut T {
        (&self.buf[off..off+mem::size_of::<T>()]).as_ptr().cast::<T>().cast_mut()
    }

    pub fn rd8(&self, off: usize) -> u8 {
        let addr = self.off::<u8>(off);
        unsafe { addr.read() }
    }

    pub fn rd8_volatile(&self, off: usize) -> u8 {
        let addr = self.off::<u8>(off);
        unsafe { addr.read_volatile() }
    }

    #[cfg(target_has_atomic = "8")]
    pub fn rd8_atomic(&self, off: usize) -> u8 {
        unsafe {
            let addr = AtomicU8::from_ptr(self.off_mut::<u8>(off));
            addr.load(Ordering::SeqCst)
        }
    }

    pub fn wr8(&self, off: usize, value: u8) {
        let addr = self.off_mut::<u8>(off);
        unsafe { addr.write(value); }
    }

    pub fn wr8_volatile(&self, off: usize, value: u8) {
        let addr = self.off_mut::<u8>(off);
        unsafe { addr.write_volatile(value); }
    }

    #[cfg(target_has_atomic = "8")]
    pub fn wr8_atomic(&self, off: usize, value: u8) {
        unsafe {
            let addr = AtomicU8::from_ptr(self.off_mut::<u8>(off));
            addr.store(value, Ordering::SeqCst)
        }
    }

    pub fn rd16(&self, off: usize) -> u16 {
        let addr = self.off::<u16>(off);
        unsafe { addr.read() }
    }

    pub fn rd16_volatile(&self, off: usize) -> u16 {
        let addr = self.off::<u16>(off);
        unsafe { addr.read_volatile() }
    }

    #[cfg(target_has_atomic = "16")]
    pub fn rd16_atomic(&self, off: usize) -> u16 {
        unsafe {
            let addr = AtomicU16::from_ptr(self.off_mut::<u16>(off));
            addr.load(Ordering::SeqCst)
        }
    }

    pub fn wr16(&self, off: usize, value: u16) {
        let addr = self.off_mut::<u16>(off);
        unsafe { addr.write(value); }
    }

    pub fn wr16_volatile(&self, off: usize, value: u16) {
        let addr = self.off_mut::<u16>(off);
        unsafe { addr.write_volatile(value); }
    }

    #[cfg(target_has_atomic = "16")]
    pub fn wr16_atomic(&self, off: usize, value: u16) {
        unsafe {
            let addr = AtomicU16::from_ptr(self.off_mut::<u16>(off));
            addr.store(value, Ordering::SeqCst)
        }
    }

    pub fn rd32(&self, off: usize) -> u32 {
        let addr = self.off::<u32>(off);
        unsafe { addr.read() }
    }

    pub fn rd32_volatile(&self, off: usize) -> u32 {
        let addr = self.off::<u32>(off);
        unsafe { addr.read_volatile() }
    }

    #[cfg(target_has_atomic = "32")]
    pub fn rd32_atomic(&self, off: usize) -> u32 {
        unsafe {
            let addr = AtomicU32::from_ptr(self.off_mut::<u32>(off));
            addr.load(Ordering::SeqCst)
        }
    }

    pub fn wr32(&self, off: usize, value: u32) {
        let addr = self.off_mut::<u32>(off);
        unsafe { addr.write(value); }
    }

    pub fn wr32_volatile(&self, off: usize, value: u32) {
        let addr = self.off_mut::<u32>(off);
        unsafe { addr.write_volatile(value); }
    }

    #[cfg(target_has_atomic = "32")]
    pub fn wr32_atomic(&self, off: usize, value: u32) {
        unsafe {
            let addr = AtomicU32::from_ptr(self.off_mut::<u32>(off));
            addr.store(value, Ordering::SeqCst)
        }
    }

    pub fn rd64(&self, off: usize) -> u64 {
        let addr = self.off::<u64>(off);
        unsafe { addr.read() }
    }

    pub fn rd64_volatile(&self, off: usize) -> u64 {
        let addr = self.off::<u64>(off);
        unsafe { addr.read_volatile() }
    }

    #[cfg(target_has_atomic = "64")]
    pub fn rd64_atomic(&self, off: usize) -> u64 {
        unsafe {
            let addr = AtomicU64::from_ptr(self.off_mut::<u64>(off));
            addr.load(Ordering::SeqCst)
        }
    }

    pub fn wr64(&self, off: usize, value: u64) {
        let addr = self.off_mut::<u64>(off);
        unsafe { addr.write(value); }
    }

    pub fn wr64_volatile(&self, off: usize, value: u64) {
        let addr = self.off_mut::<u64>(off);
        unsafe { addr.write_volatile(value); }
    }

    #[cfg(target_has_atomic = "64")]
    pub fn wr64_atomic(&self, off: usize, value: u64) {
        unsafe {
            let addr = AtomicU64::from_ptr(self.off_mut::<u64>(off));
            addr.store(value, Ordering::SeqCst)
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
